use reqwest::blocking::Client;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let client = Client::new();

    let user = "testUser".to_string();
    let passwd: Option<String> = None;

    let response = client
        .get("http://httpbin.org/get")
        .basic_auth(user, passwd)
        .send();
    println!("Response : {:?}", response);
    Ok(())
}
