use quicli::prelude::*;

use std::io;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};
use std::collections::LinkedList;

use fileutils::*;
use operations::FSOperation;
use operations::TraversOperation;

pub(crate) fn stow_path<'a>(source_path: &'a Path, target_path: &'a Path, force: bool, backup: bool, operations: &'a mut LinkedList<FSOperation>) -> io::Result<TraversOperation> {
    let target_is_directory = source_path.is_dir();
    let target_exist = target_path.exists();
    let target_is_symlink = is_symlink(target_path);
    let is_valid_symlink = check_symlink(target_path, source_path);

    let stop_if_directory = || -> io::Result<TraversOperation> {
        if target_is_directory {
            Ok(TraversOperation::StopPathRun)
        } else {
            Ok(TraversOperation::Continue)
        }
    };

    let symlink_operation = FSOperation::CreateSymlink { source: source_path.to_path_buf(), target: target_path.to_path_buf() };

    match (target_exist, target_is_symlink, target_is_directory, force) {
        (true, true, _, _) => {
            //A symbolic link already exist
            if is_valid_symlink {
                //ignore target exist if it's already the good symlink
                info!("Valid symlink {} already exist, nothing to do", target_path.display());
                Ok(TraversOperation::StopPathRun)
            } else {
                log!(Level::Warn, "Path symlink {} already exist and will be override", target_path.display());
                operations.push_back(symlink_operation);
                stop_if_directory()
            }
        }
        (true, false, false, true) => {
            // A real file already exist and force flag is set
            if backup {
                operations.push_back(FSOperation::Backup(target_path.to_path_buf()));
            } else {
                log!(Level::Warn, "Path {} already exist and will be override !", target_path.display());
                operations.push_back(FSOperation::Delete(target_path.to_path_buf()));
            }
            operations.push_back(symlink_operation);
            Ok(TraversOperation::Continue)
        }
        (true, false, false, false) => {
            // A real file already exist but force flag is not set => ERROR
            Err(Error::new(ErrorKind::Other, format!("Path {} already exist. Set -f flag to force override", target_path.display())))
        }
        (true, false, true, _) => {
            //break for existing directory
            Ok(TraversOperation::Continue)
        }
        (false, _, _, _) => {
            operations.push_back(symlink_operation);
            stop_if_directory()
        }
    }
}


#[cfg(test)]
mod test_stow {
    use super::*;
    use test_utils::*;
    use std::borrow::BorrowMut;

    const NO_FORCE: bool = false;
    const NO_BACKUP: bool = false;


    #[test]
    fn test_stow_file() {
        with_test_directories(|source: &PathBuf, target: &PathBuf| {
            let source_file = add_file_to("file.txt", source).unwrap();
            let target_file = target.join("file.txt");

            let mut operations: LinkedList<FSOperation> = LinkedList::new();
            let result = stow_path(source_file.as_path(), target_file.as_path(), NO_FORCE, NO_BACKUP, operations.borrow_mut()).expect("An error occurred when visiting directories");

            assert_eq!(result, TraversOperation::Continue);

            let mut iter = operations.iter();
            let value = iter.next().unwrap();
            assert_eq!(value, &FSOperation::CreateSymlink { source: source_file, target: target_file });
        });
    }

    #[test]
    fn test_stow_directory() {
        with_test_directories(|source: &PathBuf, target: &PathBuf| {
            let source_dir = add_directory_to("subDir", source).unwrap();
            let target_dir = target.join("subDir");

            let mut operations: LinkedList<FSOperation> = LinkedList::new();
            let result = stow_path(source_dir.as_path(), target_dir.as_path(), NO_FORCE, NO_BACKUP, operations.borrow_mut()).expect("An error occurred when visiting directories");

            assert_eq!(result, TraversOperation::StopPathRun);

            let mut iter = operations.iter();
            let value = iter.next().unwrap();
            assert_eq!(value, &FSOperation::CreateSymlink { source: source_dir, target: target_dir });
        });
    }

    #[test]
    fn test_stow_existing_file() {
        with_test_directories(|source: &PathBuf, target: &PathBuf| {
            let source_file = add_file_to("file.txt", source).unwrap();
            let target_file = add_file_to("file.txt", target).unwrap();

            let mut operations: LinkedList<FSOperation> = LinkedList::new();
            let result = stow_path(source_file.as_path(), target_file.as_path(), NO_FORCE, NO_BACKUP, operations.borrow_mut());

            // return an error
            assert!(result.is_err());
            assert!(operations.is_empty());
        });
    }

//    #[test]
//    fn test_stow_existing_directory() {
//        with_test_directories(|source: &PathBuf, target: &PathBuf| {
//            let source_file = add_directory_to("subDir", source).unwrap();
//            let target_file = add_directory_to("subDir", target).unwrap();
//
//            let mut operations: LinkedList<FSOperation> = LinkedList::new();
//            let result = stow_path(source_file.as_path(), target_file.as_path(), NO_FORCE, NO_BACKUP, operations.borrow_mut());
//
//            // return an error
//            assert!(result.is_err());
//            assert!(operations.is_empty());
//        });
//    }
}
