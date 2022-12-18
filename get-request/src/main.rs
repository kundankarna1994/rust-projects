use error_chain::error_chain;
use std::io::Read;

error_chain! {
     foreign_links {
        Fmt(::std::fmt::Error);
        HttpRequest(reqwest::Error) #[cfg(unix)];
    }
}
fn main() -> Result<()> {
    let mut res = reqwest::blocking::get("http://httbin.org/get")?;
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    println!("Status : {}", res.status());
    println!("Headers : {:#?}", res.headers());
    println!("Body : {}", body);
    Ok(())
}
