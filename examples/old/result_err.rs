//from here
//https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let _greeting_file_result = File::open("hello.txt");

    let _greeting_file = match _greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}