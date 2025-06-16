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
    let mut date: Option<main_pak::Paked> = None;
    let mut fast_date = None;
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
            "get" => {
                if fast_date.is_none() {
                    log("create fast date");
                    fast_date = Some(date.take().unwrap().fast());
                    log("fastdate created");
                }
                log("writing");
                fs::write(tags[2], fast_date.as_ref().unwrap().get(tags[1]).unwrap()).unwrap();
                log("writed");
            }
            "title" => {}
            _ => {}
        }
        if date.is_none() {
            log("start loading");
            date = Some(main_pak::Paked::load(string.trim()));
            log("loaded");
        }
    }
}
