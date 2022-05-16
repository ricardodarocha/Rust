use std::env;

fn main() {
    match env::var("USER_AGENT") {
        Ok(lang) => println!("Language code: {}", lang),
        Err(e) => println!("Couldn't read NAME ({})", e),
    };
}
