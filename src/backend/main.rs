mod lib;
mod reader;

fn main() {
    match lib::run() {
        Ok(_) => return,
        Err(err) => println!("{}", err.to_string()),
    }
}
