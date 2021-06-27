use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path_ame = "GFONT_API_KEY";
    let api_key = env::var(path_ame).expect("GFONT_API_KEY not found.");
    println!("{}", api_key);

    let web_fonts_url = "https://www.googleapis.com/webfonts/v1/webfonts?key=";

    let resp = reqwest::get(format!("{}{}", web_fonts_url, api_key)).await?;
    let body = resp.text().await?;
    let json: serde_json::Value = serde_json::from_str(&body)?;
    let obj = json.as_object().unwrap();
    for (key, value) in obj.iter() {
        println!("{}\t{}", key, value);
    }
    Ok(())
}
