use loading::Loading;
use std::process::exit;

mod cli;
use cli::Output::*;
mod wpclient;
use wpclient::WPClient;
mod extract;
use extract::Extract;
mod files;
mod test_data;
mod tests;

#[allow(unused_imports)]
use crate::test_data::*;

fn main() {
    let options = cli::options();
    let base_url = options.url;
    let download_requests = options.download;
    let wpjson = WPClient::new(base_url.clone(), download_requests);
    let mut _json_text = String::new();
    let loading = Loading::default();
    loading.text("loading    ");
    match wpjson.get_records_text() {
        Ok(o) => _json_text = o,
        Err(e) => {
            match e.url() {
                Some(s) => {
                    println!("Error connecting to {}", s.to_string());
                }
                None => {
                    println!("Error downloading content");
                }
            };
            exit(1);
        }
    }
    loading.end();
    let extract = Extract::new(_json_text, base_url.clone());
    if [Links, All].contains(&options.output) {
        let url_list = extract.url_list();
        cli::pipe_list_out(url_list);
    }
    if [Emails, All].contains(&options.output) {
        let email_list = extract.email_list();
        cli::pipe_list_out(email_list);
    }
    if [Documents, All].contains(&options.output) {
        match wpjson.get_documents_list(){
            Ok(documents_list) => cli::pipe_list_out(documents_list),
            Err(_) => ()
        }
    }
}
