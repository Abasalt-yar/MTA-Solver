use std::io::{BufReader, Write};
use std::os::windows::fs::MetadataExt;
use std::path::Path;

use colored::Colorize;
use reqwest::header::{HeaderMap, USER_AGENT};
use xml::{EventReader,reader::XmlEvent};
use crate::modules::helpers::{get_cache_folder,NumberHelpers};
pub async fn verify_resources() {
    let mut cache_folder = get_cache_folder().unwrap();
    let mut my_headers = HeaderMap::new();
    my_headers.insert(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.77 Safari/537.36".parse().unwrap());
    my_headers.insert("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8".parse().unwrap());
    my_headers.insert("Referer", "https://example.com".parse().unwrap());
    cache_folder.push_str("\\resources\\");
    println!("{}",format!("Cache Folder: ({})",cache_folder).truecolor(255,125,0));
    let client = reqwest::Client::new();
    let current_manifest = client.get("http://0.0.0.0/manifest.xml").headers(my_headers.clone()).send().await;
    if let Err(_) = current_manifest {
        println!("{}","Problem Retrieving Manifest File.".red());
        return;
    }
    let current_manifest = current_manifest.unwrap();
    println!();
    let current_manifest = current_manifest.bytes().await;
    if let Err(_) = current_manifest {
        println!("{}","Problem Processing Manifest File.".red());
        return;
    }
    let mut file = std::fs::File::create("manifest.xml").unwrap();
    let _ = file.write(&*current_manifest.unwrap());
    let _ = file.flush();
    drop(file);
    let reader = EventReader::new(BufReader::new(std::fs::File::open("manifest.xml").unwrap()));
    let mut to_download: Vec<String> = vec![];
    let mut download_size = 0;
    let mut current_path :Vec<String>= vec![];
    for e in reader {
        match e {
            Ok(XmlEvent::StartElement {name,attributes, .. }) => {

                if name.local_name.eq("folder") {

                    current_path.push(attributes.get(0).unwrap().value.to_owned());
                }else if name.local_name.to_string().eq("file") {
                    let mut file_path : String = cache_folder.clone();
                    file_path.push_str(&current_path.join("\\"));
                    file_path.push_str(&"\\");
                    file_path.push_str(&attributes.get(0).unwrap().value.to_owned());

                    let file = std::fs::File::open(&file_path);

                    if file.is_err() {

                        to_download.push(file_path.replace("\\", "/"));
                        download_size += attributes.get(1).unwrap().value.to_owned().parse::<i32>().unwrap();
                    }else {
                        let file_size = file.unwrap().metadata().unwrap().file_size();
                        if attributes.get(1).unwrap().value.to_owned().parse::<u64>().unwrap() != file_size {
                            to_download.push(file_path.replace("\\", "/"));
                            download_size += attributes.get(1).unwrap().value.to_owned().parse::<i32>().unwrap();
                        }
                    }

                }
            },
            Ok(XmlEvent::EndElement {name,..}) => {
                if name.local_name.eq("folder") {
                    current_path.pop();
                }

            },
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            _ => {
            }
        }
    }
    if download_size == 0 {
        println!("{}","All Files Verified, No Need For Changes.".green());
        return;
    }
    println!("{}",format!("{} Needs To Be Re-Downloaded, Total Size: {} byte, Downloading ... Please Wait.",to_download.len(),(download_size as usize /1024).locale_string()).truecolor(255,125,0));
    let len = to_download.len();
    for i in 0..len {
        let file = to_download.get(i).unwrap();
        let path = file.split("resources/").last().unwrap();

        let download_file = client.get(&["http://0.0.0.0/http-client-files/", &path].join("")).headers(my_headers.clone()).send().await;
        if download_file.is_err() {
            println!("{}",format!("Problem Downloading ({}) File.",path).red());
            return;
        }
        let download_file = download_file.unwrap();
        print!("\r{}",format!("File {}/{}",i,len - 1).green());
        let download_file = download_file.bytes().await;
        if download_file.is_err() {
            println!("{}",format!("Problem Downloading ({}) File.",path).red());
            return;
        }
        let download_file = download_file.unwrap();
        let _ = std::fs::create_dir_all(Path::new(&file).parent().unwrap());
        let store_file = std::fs::File::create(&file);
        if store_file.is_err() {
            println!("{}",format!("Problem Creating ({}) File.",path).red());
            return;
        }
        let mut store_file = store_file.unwrap();
        let _ = store_file.write(&*download_file);
        let _ = store_file.flush();
        drop(store_file);
    }
    let _ = std::fs::remove_file("manifest.xml");
    println!();
    println!("{}","All Files Downloaded.".green());


}