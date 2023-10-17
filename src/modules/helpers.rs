use sysinfo::{ProcessExt, System, SystemExt};
use winreg::enums::*;
use winreg::RegKey;
pub trait StringHelpers {
    fn to_hex(&self) -> u8 ;

}

pub trait NumberHelpers {
    fn locale_string(&self) -> String;
}

impl NumberHelpers for usize {
    fn locale_string(&self) -> String {
        if *self < 1000usize {
            return self.clone().to_string()
        }
        self.to_string().as_bytes().rchunks(3).rev().map(std::str::from_utf8).collect::<Result<Vec<&str>,_>>().unwrap().join(",")
    }
}
impl StringHelpers for &str {
    fn to_hex(&self) -> u8 {
        return usize::from_str_radix(&self,16).unwrap() as u8;
    }
}

pub fn get_mta_folder() -> std::io::Result<String> {
    let local_machine = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = local_machine.open_subkey("SOFTWARE\\WOW6432Node\\Multi Theft Auto: San Andreas All");
    if let Err(e) = key {
        return Err(e);
    }

    let key = key.unwrap();
    if let Err(e) = key.open_subkey("1.6") {
        return Err(e);
    }

    key.open_subkey("1.6").unwrap().get_value("Last Install Location")
}

pub fn get_mta_version() -> std::io::Result<String> {
    let local_machine = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = local_machine.open_subkey("SOFTWARE\\WOW6432Node\\Multi Theft Auto: San Andreas All");
    if let Err(e) = key {
        return Err(e);
    }

    let key = key.unwrap();
    if let Err(e) = key.open_subkey("1.6") {
        return Err(e);
    }

    key.open_subkey("1.6").unwrap().get_value("OnRestartCommand")
}

pub fn get_gta_folder() -> std::io::Result<String> {
    let local_machine = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = local_machine.open_subkey("SOFTWARE\\WOW6432Node\\Multi Theft Auto: San Andreas All");
    if let Err(e) = key {
        return Err(e);
    }

    let key = key.unwrap();
    key.open_subkey("Common").unwrap().get_value("GTA:SA Path")
}


pub fn get_cache_folder() -> std::io::Result<String> {
    let local_machine = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = local_machine.open_subkey("SOFTWARE\\WOW6432Node\\Multi Theft Auto: San Andreas All");
    if let Err(e) = key {
        return Err(e);
    }

    let key = key.unwrap();
    key.open_subkey("Common").unwrap().get_value("File Cache Path")
}

pub fn get_mta_serial() -> std::io::Result<String> {
    let local_machine = RegKey::predef(HKEY_LOCAL_MACHINE);
    let key = local_machine.open_subkey("SOFTWARE\\WOW6432Node\\Multi Theft Auto: San Andreas All");
    if let Err(e) = key {
        return Err(e);
    }

    let key = key.unwrap();
    key.open_subkey("1.6").unwrap().open_subkey("Settings").unwrap().open_subkey("general").unwrap().get_value("serial")
}

pub fn is_mta_open() -> bool {
    let system = System::new_all();
    system.processes().iter().filter(|(_pid,process)| process.name() == "Multi Theft Auto.exe").last().is_some()
}