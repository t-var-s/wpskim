use std::fs::File;
use std::io::Read;
use std::io::Write;

pub fn load_text(path: &str) -> Option<String> {
    let mut text = String::new();
    match File::open(&path) {
        Ok(mut file) => match file.read_to_string(&mut text) {
            Ok(_) => Some(text),
            Err(_) => None,
        },
        Err(_) => None,
    }
}
pub fn save_text(path: &str, json_text: &str) -> () {
    match File::create(path) {
        Ok(mut file) => {
            match file.write_all(&json_text.to_string().into_bytes()) {
                Ok(_) => (),
                Err(_) => (),
            };
        }
        Err(_) => (),
    };
}
