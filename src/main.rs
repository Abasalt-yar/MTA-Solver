
use std::path::PathBuf;
use std::process::exit;
use colored::{Colorize, control::set_virtual_terminal};

mod modules;
use modules::{helpers::{StringHelpers,NumberHelpers,get_mta_serial,get_gta_folder, get_mta_folder, get_mta_version, is_mta_open}, remove_specific_files::remove_file_with_extension, update_mta::update_mta};
use sysinfo;
use sysinfo::{CpuExt, SystemExt};
use crate::modules::remove_specific_files::{driver_signing, remove_files};
use crate::modules::verify_resources::verify_resources;


fn print_system_info() {
    let system = sysinfo::System::new_all();
    if !cfg!(windows) {
        println!("Please Run This Application On Windows...");
        exit(0);
    }
    println!("Windows name: {} ",system.long_os_version().unwrap());

    println!("Ram: {} megabytes, Free Ram: {} megabytes",(system.total_memory()as usize/1024).locale_string(),(system.free_memory()as usize /1024).locale_string());
    println!("CPU: {}",system.cpus().get(0).unwrap().brand());
    if !cfg!(target_pointer_width = "64") {
        println!("{}","Your System Is Not x64, Using 32 Bit Windows May Cause Crashes, Please Switch To X64.".red());
        std::thread::sleep(std::time::Duration::from_millis(10000));
        exit(0);
    }
}

async fn open_options () -> bool {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    println!("{}","@Written By Abasalt_Yar".truecolor(255,125,0));
    println!("{}",format!("Version: {}",env!("CARGO_PKG_VERSION")).truecolor(255,125,0));


    println!();
    if let Err(_) = get_mta_folder() {
        println!("{}", "Cannot Find MTA 1.6 Installation Folder, Please Re-Install MTA From https://mtasa.com.".red());
        tokio::time::sleep(std::time::Duration::from_millis(15000)).await;
        exit(0)
    }
    if let Err(_) = get_gta_folder() {
        println!("{}","Cannot Find GTA:SA Installation Folder, Please Launch MTA.".red());
        tokio::time::sleep(std::time::Duration::from_millis(15000)).await;
        exit(0);
    }
    if is_mta_open() {
        println!("{}","Please Close MTA.".red());
        tokio::time::sleep(std::time::Duration::from_millis(15000)).await;
        exit(0);
    }
    print_system_info();
    println!();
    println!("{}",format!("MTA Installation Folder: {} | Version: {} | Serial: {}",get_mta_folder().unwrap(),get_mta_version().unwrap().trim(),get_mta_serial().unwrap().trim()).truecolor(255,125,0));
    println!("{}",format!("GTA Installation Folder: {}",get_gta_folder().unwrap()).truecolor(255,125,0));
    println!();
    println!("{}" ,"Please Choose An Option: ".truecolor("70".to_hex(),"9a".to_hex(),"ff".to_hex()));
    println!("1: Remove .asi Files");
    println!("2: Remove d3d9.dll ");
    println!("3. Update MTA");
    println!("4. Driver Signing (Run As Administrator)");
    println!("5. Verify Resources");

    println!("0: Exit.");

    let mut line: String = Default::default();
    std::io::stdin().read_line(&mut line).unwrap();

    if line.ends_with("\r\n") {
        line.truncate(line.len() - 2);
    }

    if let Ok(option) = line.parse::<i8>() {
        match option {
            1 => {
                println!("Removing .asi Files...");
                remove_file_with_extension(PathBuf::from(get_gta_folder().unwrap()),"asi");
                println!("{}","Deleted.".green());
                tokio::time::sleep(std::time::Duration::from_millis(15000)).await;
            },
            2 => {
                println!("Removing d3d9.dll ...");
                remove_files(PathBuf::from(get_gta_folder().unwrap()),"d3d9.dll");

                println!("{}","Deleted.".green());
                tokio::time::sleep(std::time::Duration::from_millis(15000)).await;
            },
            3 => {
                update_mta().await;
                tokio::time::sleep(std::time::Duration::from_millis(15000)).await;
            },
            4 => {
                driver_signing().await;
                tokio::time::sleep(std::time::Duration::from_millis(15000)).await;
            },
            5 => {
                verify_resources().await;
                tokio::time::sleep(std::time::Duration::from_millis(15000)).await;
            },
            0 => exit(0),
            _ => return true

        }
    }

    return true;
}

#[tokio::main]
async fn main() {

    set_virtual_terminal(true).unwrap();

    loop {
        open_options().await;
    }

}
