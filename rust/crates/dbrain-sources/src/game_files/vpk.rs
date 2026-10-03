use std::{
    fs,
    io::{self, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

use super::{budget::Budget, invalid, normalize_relative, open_regular};

pub(super) struct Header {
    file: fs::File,
    tree_start: u64,
    tree_bytes: u64,
    embedded_bytes: u64,
}

impl Header {
    pub(super) fn original_file(&self) -> io::Result<fs::File> {
        let mut file = self.file.try_clone()?;
        file.seek(SeekFrom::Start(0))?;
        Ok(file)
    }
}

pub(super) struct Directory {
    pub header: Header,
    pub entries: Vec<Resource>,
}

pub(super) struct Resource {
    pub path: PathBuf,
    crc32: u32,
    pub preload: Vec<u8>,
    pub archive_index: u16,
    pub offset: u32,
    pub length: u32,
}

impl Resource {
    pub fn total_bytes(&self) -> u64 {
        self.preload.len() as u64 + u64::from(self.length)
    }
}

pub(super) fn read_directory(path: &Path, max_tree_bytes: u64) -> io::Result<Directory> {
    let mut file = open_regular(path)?;
    let size = file.metadata()?.len();
    if read_u32(&mut file)? != 0x55aa1234 {
        return Err(invalid("Ungültige VPK-Signatur"));
    }
    let version = read_u32(&mut file)?;
    let tree_bytes = u64::from(read_u32(&mut file)?);
    let (tree_start, embedded_bytes): (u64, u64) = match version {
        1 => {
            let embedded_start = 12u64
                .checked_add(tree_bytes)
                .ok_or_else(|| invalid("VPK-Überlauf"))?;
            (
                12,
                size.checked_sub(embedded_start)
                    .ok_or_else(|| invalid("VPK-Baum außerhalb der Datei"))?,
            )
        }
        2 => {
            let data_bytes = u64::from(read_u32(&mut file)?);
            let archive_md5_bytes = u64::from(read_u32(&mut file)?);
            let other_md5_bytes = u64::from(read_u32(&mut file)?);
            let signature_bytes = u64::from(read_u32(&mut file)?);
            let required = 28
                + tree_bytes
                + data_bytes
                + archive_md5_bytes
                + other_md5_bytes
                + signature_bytes;
            if required > size {
                return Err(invalid("VPK-Abschnitte außerhalb der Datei"));
            }
            (28, data_bytes)
        }
        _ => return Err(invalid("Nicht unterstützte VPK-Version")),
    };
    if tree_bytes > max_tree_bytes || tree_bytes > usize::MAX as u64 {
        return Err(invalid(
            "VPK-Verzeichnis überschreitet die Größenbegrenzung",
        ));
    }
    let tree_end = tree_start
        .checked_add(tree_bytes)
        .ok_or_else(|| invalid("VPK-Baumüberlauf"))?;
    if tree_end > file.metadata()?.len() {
        return Err(invalid("VPK-Baum außerhalb der geöffneten Datei"));
    }
    file.seek(SeekFrom::Start(tree_start))?;
    let mut budget = Budget::new();
    let mut bytes = allocate_bytes(tree_bytes as usize, &mut budget)?;
    file.read_exact(&mut bytes)?;
    let mut cursor = Cursor {
        bytes: &bytes,
        position: 0,
    };
    let mut entries = Vec::new();
    loop {
        let extension = cursor.string()?;
        if extension.is_empty() {
            break;
        }
        budget.expanded(&[extension.len()], 128).map_err(invalid)?;
        validate_piece(extension, false)?;
        loop {
            let directory = cursor.string()?;
            if directory.is_empty() {
                break;
            }
            budget.expanded(&[directory.len()], 128).map_err(invalid)?;
            validate_piece(directory, true)?;
            loop {
                let name = cursor.string()?;
                if name.is_empty() {
                    break;
                }
                budget.expanded(&[name.len()], 128).map_err(invalid)?;
                validate_piece(name, false)?;
                let crc32 = cursor.u32()?;
                let preload_length = cursor.u16()? as usize;
                let archive_index = cursor.u16()?;
                let offset = cursor.u32()?;
                let length = cursor.u32()?;
                if cursor.u16()? != 0xffff {
                    return Err(invalid("Ungültiger VPK-Eintragsabschluss"));
                }
                budget
                    .expanded(
                        &[directory.len(), name.len(), extension.len(), preload_length],
                        1536,
                    )
                    .map_err(invalid)?;
                let source_preload = cursor.take(preload_length)?;
                let mut preload = allocate_bytes(preload_length, &mut budget)?;
                preload.copy_from_slice(source_preload);
                let filename = if extension == " " {
                    name.to_owned()
                } else {
                    format!("{name}.{extension}")
                };
                let resource_path = if directory == " " {
                    PathBuf::from(filename)
                } else {
                    Path::new(&directory).join(filename)
                };
                normalize_relative(&resource_path)?;
                entries.try_reserve(1).map_err(invalid)?;
                entries.push(Resource {
                    path: resource_path,
                    crc32,
                    preload,
                    archive_index,
                    offset,
                    length,
                });
            }
        }
    }
    if cursor.position != bytes.len() {
        return Err(invalid(
            "Nicht vollständig ausgewerteter VPK-Verzeichnisbaum",
        ));
    }
    Ok(Directory {
        header: Header {
            file,
            tree_start,
            tree_bytes,
            embedded_bytes,
        },
        entries,
    })
}

fn validate_piece(value: &str, directory: bool) -> io::Result<()> {
    if value == " " {
        return Ok(());
    }
    if value.is_empty()
        || value.contains('\\')
        || value.contains(':')
        || value.contains('\0')
        || !directory && value.contains('/')
    {
        return Err(invalid("Ungültiger Pfadbestandteil im VPK"));
    }
    normalize_relative(Path::new(value)).map(|_| ())
}

pub(super) fn read_resource(
    directory_path: &Path,
    header: &Header,
    resource: &Resource,
) -> io::Result<Vec<u8>> {
    let total =
        usize::try_from(resource.total_bytes()).map_err(|_| invalid("VPK-Eintrag ist zu groß"))?;
    // Hold and validate the actual payload file before reserving or copying any bytes.
    // For embedded v2 data, the section bound also excludes trailing checksum sections.
    let payload = if resource.length > 0 {
        let end = u64::from(resource.offset)
            .checked_add(u64::from(resource.length))
            .ok_or_else(|| invalid("VPK-Eintragsüberlauf"))?;
        let (file, absolute_offset) = if resource.archive_index == 0x7fff {
            if end > header.embedded_bytes {
                return Err(invalid("VPK-Eintrag liegt außerhalb des Datenabschnitts"));
            }
            let absolute_offset = header
                .tree_start
                .checked_add(header.tree_bytes)
                .and_then(|start| start.checked_add(u64::from(resource.offset)))
                .ok_or_else(|| invalid("VPK-Eintragsüberlauf"))?;
            (header.file.try_clone()?, absolute_offset)
        } else {
            let name = directory_path
                .file_name()
                .and_then(|name| name.to_str())
                .and_then(|name| name.strip_suffix("_dir.vpk"))
                .ok_or_else(|| invalid("VPK-Verzeichnisname hat kein _dir.vpk-Suffix"))?;
            let archive_path =
                directory_path.with_file_name(format!("{name}_{:03}.vpk", resource.archive_index));
            // directory_path remains relative to the caller's held directory handle.
            (open_regular(&archive_path)?, u64::from(resource.offset))
        };
        let absolute_end = absolute_offset
            .checked_add(u64::from(resource.length))
            .ok_or_else(|| invalid("VPK-Eintragsüberlauf"))?;
        if absolute_end > file.metadata()?.len() {
            return Err(invalid("VPK-Eintrag liegt außerhalb der geöffneten Datei"));
        }
        Some((file, absolute_offset))
    } else {
        None
    };
    let mut bytes = allocate_bytes(total, &mut Budget::new())?;
    bytes[..resource.preload.len()].copy_from_slice(&resource.preload);
    if let Some((mut file, absolute_offset)) = payload {
        file.seek(SeekFrom::Start(absolute_offset))?;
        file.read_exact(&mut bytes[resource.preload.len()..])?;
    }
    if crc32(&bytes) != resource.crc32 {
        return Err(invalid(
            "VPK-CRC32 stimmt nicht mit dem Verzeichnis überein",
        ));
    }
    Ok(bytes)
}

fn allocate_bytes(count: usize, budget: &mut Budget) -> io::Result<Vec<u8>> {
    budget.charge(count).map_err(invalid)?;
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(count).map_err(invalid)?;
    // Capacity is already reserved; resize and subsequent slice copies cannot grow it.
    bytes.resize(count, 0);
    Ok(bytes)
}

fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = !0u32;
    for &byte in bytes {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0xedb88320 & mask);
        }
    }
    !crc
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    fn take(&mut self, count: usize) -> io::Result<&'a [u8]> {
        let end = self
            .position
            .checked_add(count)
            .ok_or_else(|| invalid("VPK-Baumüberlauf"))?;
        let result = self
            .bytes
            .get(self.position..end)
            .ok_or_else(|| invalid("Abgeschnittener VPK-Verzeichnisbaum"))?;
        self.position = end;
        Ok(result)
    }

    fn string(&mut self) -> io::Result<&'a str> {
        let rest = self
            .bytes
            .get(self.position..)
            .ok_or_else(|| invalid("VPK-Baumüberlauf"))?;
        let count = rest
            .iter()
            .position(|byte| *byte == 0)
            .ok_or_else(|| invalid("Nicht abgeschlossene VPK-Zeichenfolge"))?;
        let text = std::str::from_utf8(self.take(count)?).map_err(invalid)?;
        self.take(1)?;
        Ok(text)
    }

    fn u16(&mut self) -> io::Result<u16> {
        let bytes = self.take(2)?;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn u32(&mut self) -> io::Result<u32> {
        let bytes = self.take(4)?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
}

fn read_u32(reader: &mut impl Read) -> io::Result<u32> {
    let mut bytes = [0u8; 4];
    reader.read_exact(&mut bytes)?;
    Ok(u32::from_le_bytes(bytes))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package(name: &str, content: &[u8], crc: u32) -> Vec<u8> {
        let mut tree = b"txt\0scripts\0".to_vec();
        tree.extend(name.as_bytes());
        tree.push(0);
        tree.extend(crc.to_le_bytes());
        tree.extend(0u16.to_le_bytes());
        tree.extend(0x7fffu16.to_le_bytes());
        tree.extend(0u32.to_le_bytes());
        tree.extend((content.len() as u32).to_le_bytes());
        tree.extend(0xffffu16.to_le_bytes());
        tree.extend([0, 0, 0]);
        let mut bytes = Vec::new();
        bytes.extend(0x55aa1234u32.to_le_bytes());
        bytes.extend(1u32.to_le_bytes());
        bytes.extend((tree.len() as u32).to_le_bytes());
        bytes.extend(tree);
        bytes.extend(content);
        bytes
    }

    #[test]
    fn embedded_resource_is_crc_checked() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("pak01_dir.vpk");
        let content = b"root { damage 12 }";
        fs::write(&path, package("items", content, crc32(content))).unwrap();
        let archive = read_directory(&path, 1024).unwrap();
        assert_eq!(archive.entries.len(), 1);
        assert_eq!(archive.entries[0].path, Path::new("scripts/items.txt"));
        assert_eq!(
            read_resource(&path, &archive.header, &archive.entries[0]).unwrap(),
            content
        );
        fs::write(&path, package("items", content, 0)).unwrap();
        let archive = read_directory(&path, 1024).unwrap();
        assert!(read_resource(&path, &archive.header, &archive.entries[0]).is_err());
    }

    #[test]
    fn traversal_truncation_and_oversized_directory_are_rejected() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("pak01_dir.vpk");
        fs::write(&path, package("../escape", b"x", crc32(b"x"))).unwrap();
        assert!(read_directory(&path, 1024).is_err());
        let mut bytes = package("items", b"x", crc32(b"x"));
        bytes.truncate(18);
        fs::write(&path, bytes).unwrap();
        assert!(read_directory(&path, 1024).is_err());
        fs::write(&path, package("items", b"x", crc32(b"x"))).unwrap();
        assert!(read_directory(&path, 1).is_err());
    }

    #[test]
    fn shared_directory_prefix_is_budgeted_before_path_expansion() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("pak01_dir.vpk");
        let mut tree = b"txt\0".to_vec();
        tree.extend("a".repeat(100_000).as_bytes());
        tree.push(0);
        for i in 0..1000 {
            tree.extend(format!("r{i}").as_bytes());
            tree.push(0);
            tree.extend(crc32(b"").to_le_bytes());
            tree.extend(0u16.to_le_bytes());
            tree.extend(0x7fffu16.to_le_bytes());
            tree.extend(0u32.to_le_bytes());
            tree.extend(0u32.to_le_bytes());
            tree.extend(0xffffu16.to_le_bytes());
        }
        tree.extend([0, 0, 0]);
        let mut bytes = Vec::new();
        bytes.extend(0x55aa1234u32.to_le_bytes());
        bytes.extend(1u32.to_le_bytes());
        bytes.extend((tree.len() as u32).to_le_bytes());
        bytes.extend(tree);
        fs::write(&path, bytes).unwrap();
        let error = match read_directory(&path, 1024 * 1024) {
            Ok(_) => panic!("budget not enforced"),
            Err(e) => e,
        };
        assert!(error.to_string().contains(super::super::budget::EXCEEDED));
    }

    #[test]
    fn sibling_payload_stays_anchored_after_parent_replacement() {
        let temp = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        fs::create_dir(temp.path().join("root")).unwrap();
        let content = b"inside";
        let mut bytes = package("items", content, crc32(content));
        // archive_index in the single directory entry, after CRC/preload length.
        let index = 12 + b"txt\0scripts\0items\0".len() + 6;
        bytes[index..index + 2].copy_from_slice(&0u16.to_le_bytes());
        fs::write(temp.path().join("root/pak01_dir.vpk"), bytes).unwrap();
        fs::write(temp.path().join("root/pak01_000.vpk"), content).unwrap();
        fs::write(outside.path().join("pak01_000.vpk"), b"outside").unwrap();
        let held = super::super::anchored::root(&temp.path().join("root")).unwrap();
        let path = super::super::anchored::location(&held)
            .unwrap()
            .join("pak01_dir.vpk");
        let archive = read_directory(&path, 1024).unwrap();
        fs::rename(temp.path().join("root"), temp.path().join("old")).unwrap();
        std::os::unix::fs::symlink(outside.path(), temp.path().join("root")).unwrap();
        assert_eq!(
            read_resource(&path, &archive.header, &archive.entries[0]).unwrap(),
            content
        );
    }

    #[test]
    fn tiny_packages_with_huge_declared_payloads_report_bounds_gaps() {
        for index in [0x7fff, 0u16] {
            let temp = tempfile::tempdir().unwrap();
            let path = temp.path().join("pak01_dir.vpk");
            let mut bytes = package("items", b"", 0);
            assert_eq!(bytes.len(), 51);
            let record = 12 + b"txt\0scripts\0items\0".len();
            bytes[record + 6..record + 8].copy_from_slice(&index.to_le_bytes());
            bytes[record + 12..record + 16].copy_from_slice(&u32::MAX.to_le_bytes());
            fs::write(&path, bytes).unwrap();
            if index == 0 {
                fs::write(temp.path().join("pak01_000.vpk"), b"").unwrap();
            }
            let archive = read_directory(&path, 1024).unwrap();
            let error = read_resource(&path, &archive.header, &archive.entries[0]).unwrap_err();
            assert_eq!(error.kind(), io::ErrorKind::InvalidData);
            assert!(error.to_string().contains("außerhalb"));
            // The public caller must recover and retain the resource's inventory gap,
            // even when its configurable input limit permits the malicious length.
            let options: super::super::GameFileOptions =
                serde_json::from_value(serde_json::json!({
                    "root": temp.path(), "app_id": 1422450, "source_id": "fixture",
                    "observed_at": "2026-10-03T00:00:00Z", "build_id": null,
                    "manifest_id": null, "source_revision": null, "depot_id": null,
                    "language": "und", "attribution": "Testdaten", "license_name": "fixture",
                    "license_url": null, "provenance": {}, "max_file_bytes": 4294967296u64
                }))
                .unwrap();
            let mut output = Vec::new();
            let inventory = super::super::extract_game_files(&options, &mut output).unwrap();
            assert_eq!(inventory.documents, 0);
            assert!(output.is_empty());
            assert_eq!(inventory.gaps.len(), 1);
            let resource = inventory
                .files
                .iter()
                .find(|item| item.container_path.is_some())
                .unwrap();
            assert_eq!(resource.bytes, u64::from(u32::MAX));
            assert_eq!(resource.disposition, "gap");
            assert!(resource.reason.as_ref().unwrap().contains("außerhalb"));
        }
    }

    #[test]
    fn embedded_payload_checks_current_held_file_size() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("pak01_dir.vpk");
        fs::write(&path, package("items", b"abc", crc32(b"abc"))).unwrap();
        let archive = read_directory(&path, 1024).unwrap();
        FileForTest::truncate(&path, 51);
        let error = read_resource(&path, &archive.header, &archive.entries[0]).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidData);
        assert!(error.to_string().contains("geöffneten Datei"));
    }

    #[test]
    fn sparse_in_bounds_payloads_and_tree_are_budgeted_before_allocation() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("pak01_dir.vpk");
        let count = 512 * 1024 * 1024 + 1u32;
        for index in [0x7fff, 0u16] {
            let mut bytes = package("items", b"", 0);
            let record = 12 + b"txt\0scripts\0items\0".len();
            bytes[record + 6..record + 8].copy_from_slice(&index.to_le_bytes());
            bytes[record + 12..record + 16].copy_from_slice(&count.to_le_bytes());
            fs::write(&path, bytes).unwrap();
            let payload_path = if index == 0x7fff {
                path.clone()
            } else {
                let sibling = temp.path().join("pak01_000.vpk");
                fs::write(&sibling, b"").unwrap();
                sibling
            };
            FileForTest::truncate(
                &payload_path,
                u64::from(count) + if index == 0x7fff { 51 } else { 0 },
            );
            let archive = read_directory(&path, 1024).unwrap();
            let error = read_resource(&path, &archive.header, &archive.entries[0]).unwrap_err();
            assert_eq!(error.kind(), io::ErrorKind::InvalidData);
            assert_eq!(error.to_string(), super::super::budget::EXCEEDED);
        }
        let mut header = Vec::new();
        header.extend(0x55aa1234u32.to_le_bytes());
        header.extend(1u32.to_le_bytes());
        header.extend(count.to_le_bytes());
        fs::write(&path, header).unwrap();
        FileForTest::truncate(&path, 12 + u64::from(count));
        let error = read_directory(&path, u64::from(count)).err().unwrap();
        assert_eq!(error.to_string(), super::super::budget::EXCEEDED);
    }

    struct FileForTest;
    impl FileForTest {
        fn truncate(path: &Path, bytes: u64) {
            fs::OpenOptions::new()
                .write(true)
                .open(path)
                .unwrap()
                .set_len(bytes)
                .unwrap();
        }
    }

    #[test]
    fn preload_and_payload_are_combined_for_embedded_external_and_preload_only_crc() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("pak01_dir.vpk");
        let preload = b"pre";
        for index in [0x7fff, 0u16] {
            let mut bytes = package("items", b"tail", crc32(b"pretail"));
            let record = 12 + b"txt\0scripts\0items\0".len();
            bytes[record + 4..record + 6].copy_from_slice(&(preload.len() as u16).to_le_bytes());
            bytes[record + 6..record + 8].copy_from_slice(&index.to_le_bytes());
            bytes.splice(record + 18..record + 18, preload.iter().copied());
            bytes[8..12].copy_from_slice(&42u32.to_le_bytes());
            fs::write(&path, &bytes).unwrap();
            fs::write(temp.path().join("pak01_000.vpk"), b"tail").unwrap();
            let archive = read_directory(&path, 1024).unwrap();
            assert_eq!(
                read_resource(&path, &archive.header, &archive.entries[0]).unwrap(),
                b"pretail"
            );
            let mut resource = archive.entries.into_iter().next().unwrap();
            resource.crc32 = 0;
            assert!(read_resource(&path, &archive.header, &resource).is_err());
            resource.length = 0;
            resource.crc32 = crc32(preload);
            assert_eq!(
                read_resource(&path, &archive.header, &resource).unwrap(),
                preload
            );
        }
    }

    #[test]
    fn embedded_v2_payload_cannot_read_trailing_checksum_sections() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("pak01_dir.vpk");
        let v1 = package("items", b"x", crc32(b"x"));
        let mut bytes = v1[..12].to_vec();
        bytes[4..8].copy_from_slice(&2u32.to_le_bytes());
        bytes.extend(0u32.to_le_bytes()); // empty data section
        bytes.extend(1u32.to_le_bytes()); // trailing checksum section
        bytes.extend(0u32.to_le_bytes());
        bytes.extend(0u32.to_le_bytes());
        bytes.extend(&v1[12..]);
        fs::write(&path, bytes).unwrap();
        let archive = read_directory(&path, 1024).unwrap();
        let error = read_resource(&path, &archive.header, &archive.entries[0]).unwrap_err();
        assert!(error.to_string().contains("Datenabschnitts"));
    }

    #[test]
    fn crc_matches_standard_test_vector() {
        assert_eq!(crc32(b"123456789"), 0xcbf43926);
    }
}
