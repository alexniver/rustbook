use std::{
    fs::{self, remove_file, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    init();
    // let f = openfile1();
    // let f = openfile2();
    // let f = File::open(FILE_NAME).unwrap();
    // let f = File::open(FILE_NAME).expect(format!("open {:?} error", FILE_NAME).as_str());
}

const FILE_NAME: &str = "hello.txt";

fn init() {
    // remove hello.txt
    remove_file(FILE_NAME);
}

fn openfile1() -> File {
    let greeting_file_result = File::open(FILE_NAME);

    match greeting_file_result {
        Ok(f) => f,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(f) => f,
                Err(e) => panic!("create file error, {:?}", e),
            },
            other => {
                panic!("fail to open file. error : {:?}", other)
            }
        },
    }
}

fn openfile2() -> File {
    File::open(FILE_NAME).unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|e| {
                panic!("create file error, {:?}", e);
            })
        } else {
            panic!("open file error, {:?}", e);
        }
    })
}

fn readfile(filename: &str) -> Result<String, io::Error> {
    fs::read_to_string(filename)
}
