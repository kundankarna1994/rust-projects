use error_chain::error_chain;

error_chain! {
    foreign_links{
        IO(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let res = reqwest::get("http://httpbin.org/get").await?;
    println!("Status : {}", res.status());
    let body = res.text().await?;
    println!("Body {}", body);
    Ok(())
}
