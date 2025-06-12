use std::{fs::File, io::Read};

fn read_file_contents(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> Result<(), std::io::Error> {
    let file_name = "output.txt";
    let contents = read_file_contents(file_name)?;
    println!("File contents: {}", contents);
    Ok(())
}

/*
fn read_file_contents(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

fn main() {
    let file_name = "output.txt";
    let contents = read_file_contents(file_name);
    println!("File contents: {}", contents);
}






*/