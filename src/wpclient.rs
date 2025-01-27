use reqwest::blocking::Client;
use serde_json::Value;

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
        let url = self.base_url.to_owned();
        format!("{url}/wp-json/wp/v2/{records}?per_page=100")
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
    fn get_documents_json(&self) -> String {
        let documents_path = "documents.json";
        let mut json = String::new();
        let empty_json = String::from("[]");
        match files::load_text(documents_path) {
            Some(file_json) => {
                json = file_json;
            }
            None => ()
        };
        if json.len() > 0{
            return json;
        }
        let media_url = self.records_endpoint("media");
        let documents_url = format!("{media_url}&media_type=application");
        match self.client.get(documents_url).send(){
            Ok(response) => {
                match response.text(){ Ok(text) =>{ 
                    json = text;
                    if self.download_requests {
                        files::save_text(documents_path, &json);
                    }
                }, Err(_) => json = empty_json, }
            },
            Err(_) => json = empty_json,
        }
        return json;
    }
    pub fn get_documents_list(&self) -> Result<Vec<String>, serde_json::Error> {
        let mut url_list:Vec<String> = Vec::new();
        let documents_json = self.get_documents_json();
        let data:Value = serde_json::from_str(&documents_json)?;
        if let Some(list) = data.as_array(){
            for item in list{
                if let Some(guid) = item["guid"].as_object(){
                    if let Some(rendered) = guid["rendered"].as_str(){
                        url_list.push(rendered.to_string())
                    }
                }
            }
        }
        url_list.sort();
        return Ok(url_list);
    }
}
