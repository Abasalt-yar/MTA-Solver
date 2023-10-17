use std::fs::remove_file;
use std::io:: Write;
use colored::Colorize;
use sysinfo::SystemExt;
use crate::modules::helpers::{get_mta_folder, StringHelpers};
use crate::NumberHelpers;
async fn fetch_latest_link(client: &reqwest::Client) -> Result<String,bool> {
    let resp =client.get("https://nightly.multitheftauto.com/").send().await;
    if let Err(_) = resp {
        return Err(false);
    }
    let resp = resp.unwrap().text().await;
    if let Err(_) = resp {
        return Err(false);
    }
    let body = resp.unwrap();
    let selector = scraper::Selector::parse("tr.file:not([id]) > td.name > a").unwrap();
    let body =scraper::Html::parse_fragment(&body);
    let mut selector = body.select(&selector);
    if let Some(win) = sysinfo::System::new().long_os_version().unwrap().split(" ").collect::<Vec<&str>>().get(1) {
        println!("{}",format!("Windows: {}",win).green());
        if win.parse::<i32>().unwrap() < 10 {
            selector.next();
        }
    }
    if let Some(l) = selector.next() {
        return Ok(l.value().attr("href").unwrap().to_string());
    }
    return Err(false);
}

fn show_err() {
    println!("{}","There's A Problem By Fetching MTA's Installation Link.".red());
    println!("{}","Please Check Your Internet Connection.".yellow());
}

pub async fn update_mta() -> Option<String> {
    let client = reqwest::Client::new();
    let link = fetch_latest_link(&client).await;
    if let Err(_) = link {
        show_err();
        return None;
    }
    let link = link.unwrap();
    let mut version = link.split(".").collect::<Vec<_>>();
    let _ = version.pop();
    let version = version.join(".");
    println!("{}",format!("Downloading v({}) ...",version).green());

    let file = client.get(format!("{}{}","https://nightly.multitheftauto.com/",link)).send().await;
    if let Err(_) = file {
        show_err();
        return None;
    }
    let mut file = file.unwrap();
    let length: usize = file.headers().get("Content-Length").unwrap().to_str().unwrap_or("0").parse::<_>().unwrap();
    if length == 0 {
        println!("{}","Unknown Content Length Header.".red());
        show_err();
        return None;
    }
    println!("File Size: {} bytes",(length/1024).locale_string());
    let mut zip_file = std::fs::File::create(link.clone()).expect("Problem Creating Installation File.");

    let mut downloaded_size: f32 = 0.0;
    while let Some(chuck) = file.chunk().await.unwrap_or(None) {
        zip_file.write(&*chuck).expect("Problem Writing Installation File");
        downloaded_size += chuck.len() as f32;
        print!("\r{}",format!("{:.2}% Downloaded.",(downloaded_size/length as f32)*100.0).green());
    };
    if (length/1024).locale_string().trim().eq((downloaded_size as usize/1024).locale_string().trim()) == false {
        println!("\n{}",format!("Corrupted File Detected. Original Size: {} Bytes, Download File's Size: {} Bytes",(length/1024).locale_string(),(downloaded_size as usize/1024).locale_string()).red());
        return None;
    }
    zip_file.flush().expect("Problem Saving Installation File");

    println!("\n{}",format!("Installing In ({})",get_mta_folder().unwrap()).truecolor("70".to_hex(),"9a".to_hex(),"ff".to_hex()));
    println!("File Name: ({})",link);
    let _ = drop(zip_file);
    let _ = std::process::Command::new("cmd").args(&["/C","start","/wait",&link,"/S"]).spawn().unwrap().wait().unwrap();
    let _ = remove_file(link);
    println!("{}","Installed. Please Re-Launch MTA To Effect The Changes.".green());
    return Some(version);
}