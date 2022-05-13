use reqwest::header::{USER_AGENT, HeaderMap, CONTENT_TYPE};

fn main() {

    let client = reqwest::blocking::Client::new();
    let base_url = "https://api.skyhub.com.br/";
    let resource = "products";

    println!("connecting to {} {} ...", base_url, resource);
    
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, "Sistemais Informática".parse().unwrap());
    headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
    headers.insert("Accept", "application/json".parse().unwrap());
    headers.insert("X-Api-Key", "DSYbFkq8-QqkLVeyCuUn".parse().unwrap());
    headers.insert("X-User-Email", "caneschi@sistemais.com.br".parse().unwrap());
    headers.insert("X-accountmanager-Key", "s8m0sOzRP4".parse().unwrap());

    let request = client
        .get(format!("{}{}", base_url, resource))
        .headers(headers)  //how do I get my 'map' of headers into here??
        .send().expect("Could not send request")
        .text().expect("Could not parse response");
    println!("{:#?}", request);

    //  request.copy_to(&mut std::io::stdout()).unwrap();
}
