use swapi::models::{APIResponse, RawCharacter};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://swapi.dev/api/people").await?;
    let api_response: APIResponse = response.json().await?;

    let mut raw_characters: Vec<RawCharacter> = Vec::new();

    raw_characters.extend(api_response.results);

    println!("{:?}", raw_characters);
    Ok(())
}
