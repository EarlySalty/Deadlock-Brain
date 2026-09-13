use std::{
    fs::{self, OpenOptions},
    io::{self, Write},
    path::Path,
};

pub fn write_new(path: impl AsRef<Path>, bytes: &[u8]) -> io::Result<()> {
    let path = path.as_ref();
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    if let Err(error) = file.write_all(bytes).and_then(|()| file.sync_all()) {
        drop(file);
        if let Err(cleanup) = fs::remove_file(path) {
            return Err(io::Error::new(
                error.kind(),
                format!("{error}; unvollständige Ausgabe konnte nicht entfernt werden: {cleanup}"),
            ));
        }
        return Err(error);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn existing_output_is_never_overwritten() {
        let path = std::env::temp_dir().join(format!("dbrain-exclusive-{}", std::process::id()));
        super::write_new(&path, b"original").unwrap();
        assert!(super::write_new(&path, b"replacement").is_err());
        assert_eq!(std::fs::read(&path).unwrap(), b"original");
        std::fs::remove_file(path).unwrap();
    }
}
