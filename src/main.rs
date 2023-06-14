use error_chain::error_chain;
use std::fs::File;
use std::io::Read;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        ParseInt(std::num::ParseIntError);
    }

    errors {
        CustomError(msg: String) {
            description("Custom error")
            display("Custom error: {}", msg)
        }
    }
}

fn read_file() -> Result<String> {
    let mut file = File::open("file.txt")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() {
    match read_file() {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => println!("Error: {}", err),
    }
}
