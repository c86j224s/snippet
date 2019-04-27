use std::fs;
use std::io::prelude::*;


/*
파일 내용 형식 : 

한글이름(EnglishName) <email@addr.ess>
한글이름(EnglishName) <email@addr.ess>
한글이름(EnglishName) <email@addr.ess>
한글이름(EnglishName) <email@addr.ess>
*/
fn main() {
    let mut file = fs::File::open("test.txt").unwrap();

    let mut txt = String::new();
    file.read_to_string(&mut txt).unwrap();

    for line in txt.split(|c| c == '\r' || c == '\n') {
        let mut splitted = line.split(|c| c == '<' || c == '>');

        let name = splitted.next().unwrap().trim();
        let email = splitted.next().unwrap().trim();

        println!("name[{}], email[{}]", name, email);
    }


    println!("Hello, world!");
}
