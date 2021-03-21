
mod parse;
mod exception;

fn main() {
    if let Err(e) = parse::parse_command() {
        println!("{}", e.message());
    } 
}

