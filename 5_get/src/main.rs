use reqwest::header::{HeaderMap, CONTENT_TYPE, USER_AGENT};

extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    // let _user_agent = env::var("USER_AGENT").unwrap();
    let _user_agent = "Adsad";
    // let _api_key = env::var("API_KEY").unwrap();
    let _api_key = "adadada";
    // let _account_manager = env::var("ACCOUNT_MANAGER").unwrap();
    let _account_manager = "addadada";
    let _user_email = env::var("USER_EMAIL");

    let client = reqwest::blocking::Client::new();
    let base_url = "https://api.skyhub.com.br/";
    let resource = "products";

    println!("connecting to {} {} ...", base_url, resource);
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, _user_agent.parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("X-Api-Key", _api_key.parse().unwrap());
    headers.insert("X-User-Email", _user_email.parse().unwrap());
    headers.insert("X-accountmanager-Key", _account_manager.parse().unwrap());

    let request = client
        .get(format!("{}{}", base_url, resource))
        .headers(headers) //how do I get my 'map' of headers into here??
        .send()
        .expect("Could not send request")
        .text()
        .expect("Could not parse response");
    println!("{:#?}", request);

    //  request.copy_to(&mut std::io::stdout()).unwrap();

    
    
    
}
