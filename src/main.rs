use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
//use std::collections::HashMap;

#[derive(Debug)]
enum MyError {
    AttrError(String),
    HttpError(reqwest::StatusCode),
    ReqwestError(reqwest::Error),
}

enum MyPredicate<'a> {
    OnlyName(Name<&'a str>),
    WithAttr(Attr<&'a str, &'a str>),
}

fn find_element_by_attr(tag_name: &str, attr_name: Option<&str>, attr_value: Option<&str>, document: &Document) -> Result<(), MyError> {
    let predicate = match (attr_name, attr_value) {
        (Some(name), Some(value)) => MyPredicate::WithAttr(Attr(name, value)),
        (Some(_), None) | (None, Some(_)) => {
            return Err(MyError::AttrError("Either both or neither of attr_name and attr_value should be provided.".to_string()));
        }
        (None, None) => MyPredicate::OnlyName(Name(tag_name)),
    };

    match predicate {
        MyPredicate::OnlyName(name_predicate) => {
            for node in document.find(name_predicate) {
                println!("Element: {}", node.text());
            }
        }
        MyPredicate::WithAttr(attr_predicate) => {
            let custom_predicate = Name(tag_name).and(attr_predicate);
            for node in document.find(custom_predicate) {
                println!("Element: {}", node.text());
            }
        }
    }

    Ok(())
}

fn main() -> Result<(), MyError> {
    let url = "https://www.peintures-saint-luc.com/distributeur/albertini-beausoleil-06/";
    let client = Client::new();
    let response = client.get(url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
        .send()
        .map_err(MyError::ReqwestError)?;

    if response.status().is_success() {
        let body = response.text().map_err(MyError::ReqwestError)?;
        let document = Document::from(body.as_str());
        let result = find_element_by_attr("h1", None, Some("entry-title"), &document);

        match result {
            Ok(_) => Ok(()),
            Err(MyError::AttrError(msg)) => {
                Err(MyError::AttrError(msg))
            },
            Err(err) => Err(err),
        }
    } else {
        let status = response.status();
        println!("Error: {}", status);
        Err(MyError::HttpError(status))
    }
}
