use std::fs;
use std::io::Write;

pub fn load_text(path: &str) -> Option<String> {
    fs::read_to_string(path).ok()
}
pub fn save_text(path: &str, json_text: &str) -> () {
    match fs::File::create(path) {
        Ok(mut file) => {
            match file.write_all(&json_text.to_string().into_bytes()) {
                Ok(_) => (),
                Err(_) => (),
            };
        }
        Err(_) => (),
    };
}
