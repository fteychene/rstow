use super::*;
use std::fs::*;

pub const TEST_SOURCE: &'static str = "/tmp/source";
pub const TEST_TARGET: &'static str = "/tmp/target";

pub fn build_source_target_directories() -> io::Result<()> {
    println!("Create test directories");

    let source: PathBuf = PathBuf::from(TEST_SOURCE);
    let target: PathBuf = PathBuf::from(TEST_TARGET);
    create_dir_all(source.as_path());
    create_dir_all(target.as_path());
    File::create(source.join("file.txt").as_path());
    Ok(())
}

pub fn clear_directories() -> io::Result<()> {
    println!("Clean test directories");
    let source: PathBuf = PathBuf::from(TEST_SOURCE);
    let target: PathBuf = PathBuf::from(TEST_TARGET);
    remove_dir_all(target);
    remove_dir_all(source);
    Ok(())
}

pub fn add_file_to(name: &str, path: &PathBuf) -> io::Result<PathBuf> {
    let file_path = path.join(name);
    println!("Add file {} in {} directory", file_path.display(), path.display());
    File::create(file_path.as_path());
    Ok(file_path)
}

pub fn add_directory_to(name: &str, path: &PathBuf) -> io::Result<PathBuf> {
    let dir_path = path.join(name);
    println!("Add directory {} in {} directory", dir_path.display(), path.display());
    create_dir_all(dir_path.as_path());
    Ok(dir_path)
}

pub fn with_test_directories(test: impl FnOnce(&PathBuf, &PathBuf) -> ()) {
    build_source_target_directories();

    let source: PathBuf = PathBuf::from(TEST_SOURCE);
    let target: PathBuf = PathBuf::from(TEST_TARGET);

    test(&source, &target);

    clear_directories();
}