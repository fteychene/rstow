use super::*;
use std::fs::*;

const TEST_SOURCE: &'static str = "/tmp/source";
const TEST_TARGET: &'static str = "/tmp/target";

fn build_source_target_directories() -> io::Result<()> {
    println!("Create test directories");

    let source: PathBuf = PathBuf::from(TEST_SOURCE);
    let target: PathBuf = PathBuf::from(TEST_TARGET);
    create_dir_all(source.as_path());
    create_dir_all(target.as_path());
    File::create(source.join("file.txt").as_path());
    Ok(())
}

fn clear_directories() -> io::Result<()> {
    println!("Clean test directories");
    let source: PathBuf = PathBuf::from(TEST_SOURCE);
    let target: PathBuf = PathBuf::from(TEST_TARGET);
    remove_dir_all(target);
    remove_dir_all(source);
    Ok(())
}


#[test]
fn test_basic_stow() {
    build_source_target_directories();
    let source: PathBuf = PathBuf::from(TEST_SOURCE);
    let target: PathBuf = PathBuf::from(TEST_TARGET);
    let force_flag = false;
    let backup_flag = false;
    let unstow_flag = false;
    let mut operations: LinkedList<io::Result<FSOperation>> = LinkedList::new();

    visit(source.as_path(), target.as_path(), force_flag, backup_flag, unstow_flag, operations.borrow_mut()).expect("An error occurred when visiting directories");

    let mut iter = operations.iter();

    let value = iter.next().unwrap().as_ref().unwrap();
    assert!(value == &FSOperation::CreateSymlink { source: source.join("file.txt"), target: target.join("file.txt") });
    clear_directories();
}