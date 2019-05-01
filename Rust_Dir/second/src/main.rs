use std::fs;
use std::io;
use std::result::Result;
use std::io::prelude::*;

use regex;


/*
파일 내용 형식 : 

한글이름(EnglishName) <email@addr.ess>
한글이름(EnglishName) <email@addr.ess>
한글이름(EnglishName) <email@addr.ess>
한글이름(EnglishName) <email@addr.ess>
*/
fn test1() -> Result<(), io::Error> {
    println!("* test1 * ");

    let mut file = fs::File::open("test.txt")?;

    let mut txt = String::new();
    file.read_to_string(&mut txt).unwrap();

    for line in txt.split(|c| c == '\r' || c == '\n') {
        if line.len() < 1 {
            continue;
        }

        let mut splitted = line.split(|c| c == '<' || c == '>');

        let name = match splitted.next() {
            Some(some) => some,
            _ => { panic!("no name");  }
        };
        let name = name.trim();
        
        let email = match splitted.next() {
            Some(some) => some,
            _ => { panic!("no email"); }
        };
        let email = email.trim();

        println!("name[{}], email[{}]", name, email);
    }

    Ok(())
}

fn test2() -> Result<(), io::Error> {
    println!("* test2 *");

    let mut file = fs::File::open("test.txt")?;

    let mut txt = String::new();
    file.read_to_string(&mut txt).unwrap();

    let re = match regex::Regex::new(r"(\w+\([\w\s]+\)) <(\w+@\w+\.\w+)>") {
        Ok(ok) => ok,
        Err(err) => { println!("{}", err); panic!(""); }
    };
    for line in txt.split(|c| c == '\r' || c == '\n') {
        if line.len() < 1 {
            continue;
        }

        for cap in re.captures_iter(line) {
            println!("name[{0}], email[{1}]", &cap[1], &cap[2]);
        }
    }

    Ok(())
}


fn main() {
    test1().expect("");
    test2().expect("");

}
