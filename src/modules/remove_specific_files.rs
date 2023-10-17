
use std::fs;
use std::fs::remove_file;
use std::os::windows::process::ExitStatusExt;
use std::path::PathBuf;
use std::process::{ExitStatus, Stdio};
use colored::Colorize;

pub fn remove_file_with_extension(dir: PathBuf, extension: &str) {
    let folder = fs::read_dir(dir).unwrap();
    for path in folder {
        let path = path.unwrap();
        let metadata = path.metadata().unwrap();
        if metadata.is_dir() {
            remove_file_with_extension(path.path(),extension);
        }
        if path.file_name().into_string().unwrap().split('.').last().unwrap_or("") == extension {
            let _ = remove_file(path.path());
        }
    }
}

pub fn remove_files(dir: PathBuf,file_name: &str) {
    let folder = fs::read_dir(dir).unwrap();
    for path in folder {
        let path = path.unwrap();
        let metadata = path.metadata().unwrap();
        if metadata.is_dir() {
            remove_files(path.path(),file_name);
        }
        if path.file_name().into_string().unwrap() == file_name {
            let _ = remove_file(path.path());
        }
    }
}

pub async fn driver_signing () -> bool{

    let _ = std::process::Command::new("BCDEDIT").args(&["/set","NOINTEGRITYCHECKS","OFF"]).stdout(Stdio::null()).spawn().unwrap().wait().unwrap();
    let output = std::process::Command::new("BCDEDIT").args(&["/set","TESTSIGNING","OFF"]).stdout(Stdio::null()).spawn().unwrap().wait().unwrap();
    if output != ExitStatus::from_raw(0) {
        println!("{}","Please Re-Launch This Application As Administrator (Run As Administrator).".red());
        tokio::time::sleep(std::time::Duration::from_millis(20000)).await;
        return false;
    }
    println!("{}","Please Restart Your System.".green());
    return true;
}