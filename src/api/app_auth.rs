use lazy_static::lazy_static;
use std::{fs::File, io::Read};

lazy_static! {
    pub static ref API_KEY: String = get_api_key();
}

fn get_api_key() -> String {
    let mut file = match File::open("resources/secret/api-key.txt") {
        Ok(f) => f,
        Err(_) => {
            panic!("No api key found! Are you sure it's at resources/secret/api-key.txt?");
        }
    };

    let mut api_key = String::new();

    match file.read_to_string(&mut api_key) {
        Ok(_) => (),
        Err(why) => println!("Error reading key: {}", why),
    }

    api_key.trim().to_string()
}
