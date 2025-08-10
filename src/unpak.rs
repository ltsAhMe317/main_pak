use std::{fs, path::Path};

use main_pak;

fn log(str: &str) {
    println!("[log]{}", str);
}
fn read(str: &mut String) {
    std::io::stdin().read_line(str).unwrap();
}

fn main() {
    let mut exit = false;
    let mut string = String::new();
    let mut date: Option<main_pak::Pak> = None;
    while !exit {
        string.clear();
        read(&mut string);
        let tags: Vec<&str> = string.trim().split(' ').collect();
        // for what in tags.iter() {
        //     println!("{what}");
        // }
        match tags[0] {
            "q" | "exit" | "quit" => {
                log("exit");
                exit = true;
                continue;
            }
            "list" => {
                log("start list");
                let date=date.as_ref().unwrap();
                for date in date.date.iter(){
                   println!("{:?}",date.0);
                } 
            }
            _ => {}
        }
        if date.is_none() {
            log("start loading");
            date = Some(main_pak::Pak::load(string.trim()));
            log("loaded");
        }
    }
}
