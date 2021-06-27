use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");

    let path_ame = "GFONT_API_KEY";
    let api_key = env::var(path_ame).expect("GFONT_API_KEY not found.");
    println!("{}", api_key);

    let web_fonts_url = "https://www.googleapis.com/webfonts/v1/webfonts?key=";

    println!("{}", format!("{}{}", web_fonts_url, api_key));
    let resp = reqwest::get(format!("{}{}", web_fonts_url, api_key))
        .await?
        .json::<HashMap<String, String>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())

}
