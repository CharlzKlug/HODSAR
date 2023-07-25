// use std::io::{Error, ErrorKind};

fn find(needle: u16, haystack: Vec<u16>) -> Option<usize> {
    haystack.iter().position(|&x| x == needle)
}

fn read_file(path: &str) -> Result<String, u16> {
    if path == "/good/file.txt" {
	Ok("Good".to_string())
    } else {
	Err(1)
    }
}

fn main() {
    println!("Hello, world!");

    match find(2, vec![1, 3, 4, 5]) {
	Some(_) => println!("Found!"),
	None => println!("Not found :(")
    }

    if let Some(result) = find(2, vec![1,2,3,4]) {
	println!("Found!");
	println!("{}", result);
    }

    match read_file("/tmp/not/a/file") {
	Ok(content) => println!("{}", content),
	Err(error) => println!("Oh no! Error is {}", error)
    }

    match read_file("/good/file.txt") {
	Ok(content) => println!("{}", content),
	Err(error) => println!("Oh no! Error is {}", error)
    }

    if let Ok(content) = read_file("/good/file.txt") {
	println!("{}", content);
    }

}
