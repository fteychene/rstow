
use quicli::prelude::*;

use std::io;
use std::io::{Error, ErrorKind};
use std::fs::{self};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::os::unix::fs::symlink;

pub fn create_symlink(source_path: &Path, target_path: &Path) -> io::Result<()> {
    if cfg!(target_family = "unix") {
        info!("create symbolic link {} -> {}", source_path.display(), target_path.display());
        symlink(source_path, target_path)
    } else {
        Err(Error::new(ErrorKind::Other, "OS not supported"))
    }
}

pub fn build_backup_path(path: &Path) ->io::Result<PathBuf> {
    let file_name = path.file_name()
        .and_then(|x: &OsStr| x.to_str())
        .expect("Unable to get filename");

    let parent_path = path.parent().expect("Unable to get parent directory");
    Ok(parent_path.join("backup-".to_owned()+file_name))
}

pub fn backup_path(path: &Path) -> io::Result<()> {
    let backup_path = build_backup_path(path)?;

    info!("backup {} into {}", path.display(), backup_path.as_path().display());
    fs::rename(path, backup_path.as_path())
}

pub fn restore_path(backup: &Path, target: &Path) -> io::Result<()> {
    info!("restore backup {} into {}", backup.display(), target.display());
    fs::rename(backup, target)
}

pub fn delete_path(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        info!("delete directory recursively {}", path.display());
        fs::remove_dir_all(path)
    } else {
        info!("delete file {}", path.display());
        fs::remove_file(path)
    }
}

pub fn is_symlink(path: &Path) -> bool {
    match  path.symlink_metadata() {
        Ok(data) => data.file_type().is_symlink(),
        Err(_e) => false
    }
}

pub fn check_symlink(symlink_path: &Path, valid_dest: &Path) -> bool {
    match fs::read_link(symlink_path) {
        Ok(real) => valid_dest.eq(real.as_path()),
        Err(_e) => false
    }
}
