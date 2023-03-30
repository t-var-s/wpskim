use std::process::exit;
use loading::Loading;

mod cli;
mod wpclient;
use wpclient::WPClient;
mod extract;
use extract::Extract;
mod files;
mod test_data;

#[allow(unused_imports)]
use crate::test_data::*;

fn main(){
    let options = cli::options();
    let base_url = options.url;
    let download_requests = options.download;
    let wpjson = WPClient::new(base_url.clone(), download_requests);
    let mut _json_text = String::new();
    let loading = Loading::default();
    loading.text("loading");
    match wpjson.get_records_text(){
        Ok(o) => _json_text = o,
        Err(e) => {
            match e.url(){
                Some(s) => { println!("Error connecting to {}", s.to_string()); }
                None => { println!("Error downloading content"); }
            };
            exit(1);
        }
    }
    loading.end();
    let extract = Extract::new(_json_text, base_url.clone());
    if options.output != cli::Output::Emails{
        let url_list = extract.url_list();
        cli::pipe_list_out(url_list);
    }
    if options.output != cli::Output::Links{
        let email_list = extract.email_list();
        cli::pipe_list_out(email_list);
    }
}

#[test]
fn test_extract(){
    let extract = Extract::new(String::from(JSON_TEXT), String::from("https://wordpress.org"));
    let url_list = extract.url_list();
    let email_list = extract.email_list();
    assert_eq!(url_list, [
        "https://wp.me/P1OHUb-4pI", 
        "https://www.bluehost.com/wordpress-hosting", 
        "https://www.dreamhost.com/wordpress-hosting/", 
        "https://www.siteground.com/wordpress-hosting.htm",
    ]);
    assert_eq!(email_list, ["hosting@wordpress.org"]);
}



