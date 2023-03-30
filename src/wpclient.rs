use reqwest::blocking::Client;

use crate::files;

pub struct WPClient {
    base_url: String,
    client: Client,
    download_requests: bool,
}
impl WPClient {
    pub fn new(base_url: String, download_requests: bool) -> WPClient {
        WPClient {
            base_url,
            client: Client::new(),
            download_requests,
        }
    }
    fn records_endpoint(&self, records: &str) -> String {
        self.base_url.to_owned() + "/wp-json/wp/v2/" + records
    }
    pub fn get_records_text(&self) -> Result<String, reqwest::Error> {
        let mut text = String::new();
        for name in vec!["posts", "pages"] {
            let path = name.to_owned() + ".json";
            match files::load_text(&path) {
                Some(file_text) => {
                    text = text + &file_text;
                    continue;
                }
                None => (),
            };
            let url = self.records_endpoint(name);
            let response_text = self.client.get(&url).send()?.text()?;
            text = text + &response_text;
            if self.download_requests {
                files::save_text(&path, &text);
            }
        }
        return Ok(text);
    }
}
