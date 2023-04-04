#[cfg(test)]
mod tests {
    use crate::test_data::JSON_TEXT;
    use crate::Extract;
    use crate::WPClient;
    #[test]
    fn extract() {
        let extract = Extract::new(
            String::from(JSON_TEXT),
            String::from("https://wordpress.org"),
        );
        let url_list = extract.url_list();
        let email_list = extract.email_list();
        assert_eq!(
            url_list,
            [
                "https://wp.me/P1OHUb-4pI",
                "https://www.bluehost.com/wordpress-hosting",
                "https://www.dreamhost.com/wordpress-hosting/",
                "https://www.siteground.com/wordpress-hosting.htm",
            ]
        );
        assert_eq!(email_list, ["hosting@wordpress.org"]);
    }
    #[test]
    fn unreachable_url(){
        let base_url = String::from("https://w6pmcxbvbsyekqw6pmcx7jxckk.com");
        let wpjson = WPClient::new(base_url.clone(), false);
        match wpjson.get_records_text() {
            Ok(_) => panic!("Ok result for non-existing URL"),
            Err(e) => {
                match e.url() {
                    Some(s) => {
                        assert!(s.to_string().contains(&base_url));
                    }
                    None => {
                        panic!("Found no URL to output for error")
                    }
                };
            }
        }
    }
}
