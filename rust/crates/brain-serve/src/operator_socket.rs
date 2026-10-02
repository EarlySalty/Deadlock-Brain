//! Der Operatorweg akzeptiert nur den freigegebenen lokalen Betriebssystembenutzer.
use crate::Error;
use std::{
    os::unix::fs::{MetadataExt, PermissionsExt},
    path::{Component, Path, PathBuf},
    time::Duration,
};
use tokio::net::{unix::SocketAddr, UnixListener, UnixStream};

pub(crate) struct OperatorListener {
    listener: UnixListener,
    uid: u32,
}

pub(crate) struct SocketGuard {
    path: PathBuf,
    device: u64,
    inode: u64,
}

impl Drop for SocketGuard {
    fn drop(&mut self) {
        if std::fs::symlink_metadata(&self.path)
            .is_ok_and(|metadata| metadata.dev() == self.device && metadata.ino() == self.inode)
        {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

fn parent_uid(path: &Path) -> Result<u32, Error> {
    let parent = path
        .parent()
        .ok_or(Error::ConfigInvalid("operator_socket"))?;
    let metadata = std::fs::symlink_metadata(parent).map_err(|_| Error::Bind)?;
    if !metadata.is_dir() || metadata.mode() & 0o777 != 0o700 {
        return Err(Error::ConfigInvalid("operator_directory"));
    }
    let uid = metadata.uid();
    let mut current = PathBuf::new();
    let mut after_sticky = false;
    for component in parent.components() {
        if !matches!(component, Component::RootDir | Component::Normal(_)) {
            return Err(Error::ConfigInvalid("operator_directory"));
        }
        current.push(component);
        let metadata = std::fs::symlink_metadata(&current).map_err(|_| Error::Bind)?;
        if !metadata.is_dir()
            || ![0, uid].contains(&metadata.uid())
            || (after_sticky && (metadata.uid() != uid || metadata.mode() & 0o077 != 0))
        {
            return Err(Error::ConfigInvalid("operator_directory"));
        }
        after_sticky = metadata.mode() & 0o022 != 0;
        if after_sticky && !(metadata.uid() == 0 && metadata.mode() & 0o1000 != 0) {
            return Err(Error::ConfigInvalid("operator_directory"));
        }
    }
    Ok(uid)
}

pub(crate) fn bind(path: &Path) -> Result<(OperatorListener, SocketGuard), Error> {
    let uid = parent_uid(path)?;
    // Vorhandene Sockets werden nie entfernt. Ein fremder oder alter Listener blockiert den Start.
    let listener = UnixListener::bind(path).map_err(|_| Error::Bind)?;
    let metadata = std::fs::symlink_metadata(path).map_err(|_| Error::Bind)?;
    let guard = SocketGuard {
        path: path.to_owned(),
        device: metadata.dev(),
        inode: metadata.ino(),
    };
    if metadata.uid() != uid {
        return Err(Error::ConfigInvalid("operator_owner"));
    }
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
        .map_err(|_| Error::Bind)?;
    uplink_infisical_transport::validate_socket(path, uid)
        .map_err(|_| Error::ConfigInvalid("operator_socket"))?;
    Ok((OperatorListener { listener, uid }, guard))
}

impl axum::serve::Listener for OperatorListener {
    type Io = UnixStream;
    type Addr = SocketAddr;

    async fn accept(&mut self) -> (UnixStream, SocketAddr) {
        loop {
            match self.listener.accept().await {
                Ok((stream, address))
                    if stream.peer_cred().is_ok_and(|peer| peer.uid() == self.uid) =>
                {
                    return (stream, address);
                }
                Ok(_) => tokio::task::yield_now().await,
                Err(_) => tokio::time::sleep(Duration::from_millis(100)).await,
            }
        }
    }

    fn local_addr(&self) -> std::io::Result<SocketAddr> {
        self.listener.local_addr()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::serve::Listener;

    #[tokio::test]
    async fn private_rechte_und_tatsaechliche_peer_uid_werden_geprueft() {
        let directory = tempfile::tempdir().unwrap();
        std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o700)).unwrap();
        let path = directory.path().join("operator.sock");
        let (mut listener, guard) = bind(&path).unwrap();
        assert_eq!(std::fs::metadata(&path).unwrap().mode() & 0o777, 0o600);
        let client = UnixStream::connect(&path).await.unwrap();
        let (stream, _) = listener.accept().await;
        assert_eq!(stream.peer_cred().unwrap().uid(), listener.uid);
        drop((stream, client));
        listener.uid = listener.uid.wrapping_add(1);
        let _wrong_peer = UnixStream::connect(&path).await.unwrap();
        assert!(
            tokio::time::timeout(Duration::from_millis(30), listener.accept())
                .await
                .is_err()
        );
        assert!(bind(&path).is_err());
        drop(listener);
        drop(guard);
        assert!(!path.exists());
        std::fs::set_permissions(directory.path(), std::fs::Permissions::from_mode(0o750)).unwrap();
        assert!(bind(&path).is_err());
    }
}
