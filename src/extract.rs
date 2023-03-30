use regex::Regex;
use std::collections::HashSet;
use url::Url;

pub struct Extract {
    text: String,
    excluded_url: String,
}
impl Extract {
    pub fn new(text: String, excluded_url: String) -> Extract {
        Extract { text, excluded_url }
    }
    pub fn url_list(&self) -> Vec<String> {
        let mut url_set = HashSet::new();
        let url_regex = Regex::new(r#"(https?:\\/\\/[^ "'{<;]+)"#).expect("failing at regexing");
        for cap in url_regex.captures_iter(&self.text) {
            let url = cap[1].replace("\\", "");
            if url.contains(&self.excluded_url) || url == "https://api.w.org/" {
                continue;
            }
            url_set.insert(url);
        }
        let mut url_list: Vec<String> = Vec::new();
        for u in url_set.into_iter() {
            match Url::parse(&u) {
                Err(_) => {}
                Ok(_) => {
                    url_list.push(u);
                }
            }
        }
        url_list.sort();
        url_list
    }
    pub fn email_list(&self) -> Vec<String> {
        let mut email_set = HashSet::new();
        let email_regex = Regex::new(r#"([^ "'{>:]+@[^ "'{<]+)"#).unwrap();
        for cap in email_regex.captures_iter(&self.text) {
            let email = cap[1].replace("\\", "");
            if !email.contains(".") {
                continue;
            }
            email_set.insert(email);
        }
        let mut email_list = Vec::from_iter(email_set.into_iter());
        email_list.sort();
        email_list
    }
}
