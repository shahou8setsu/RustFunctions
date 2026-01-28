use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*; /*io::prelude全て*/

#[allow(unused)]
fn main() {
    //file r/w
    read_arg();
    read_file();
    write_file();

    //pattern match
    use regex::Regex;
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    
    println!("{}", match re.is_match("2014-01-01") {
        true => "Ok",
        false => "Ng",
    });
}

fn read_arg() {
    //arg list
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path)
          .expect("Should have been able to read the file");
    println!("Hello, world!");
    println!("{contents}\n");
}

fn read_file() {

    let mut f = File::open("poem.txt").expect("No such file...");
    let mut contents = String::new();
    f.read_to_string(&mut contents).unwrap();
    // println!("With text:\n{}", contents);
    match_content(contents);
    match_content(print_today());
}

fn write_file() {
    print_today();
    print_date();
}

fn match_content(content: String) {
    println!("{}", content);
}


fn print_today() -> String {
    
    let s = String::from("hoge");
    println!("{}",s);
    s
}
#[allow(unused)]
fn print_date() {
    //use chrono::{Utc, Local, DateTime, Date};
    use chrono::{Utc, Local, DateTime};

    let utc_datetime: DateTime<Utc> = Utc::now();
    //deprecated
    //let utc_date: DateTime<Utc> = Utc::today();

    println!("{}", utc_datetime);
    //println!("{}", utc_date);

    let local_datetime: DateTime<Local> = Local::now();

    //let local_date: DateTime<Local> = Local::today();

    println!("{}", local_datetime);
    //println!("{}", local_date);

}
