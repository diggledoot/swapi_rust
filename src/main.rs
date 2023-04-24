#![allow(unused)]

use std::{fs::File, io::Write};

use reqwest::Response;
use serde_json::Error;
use swapi::models::{APIResponse, RawCharacter};

const SWAPI_PEOPLE_URL: &str = "https://swapi.dev/api/people";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_characters: Vec<RawCharacter> = get_characters(SWAPI_PEOPLE_URL.to_string()).await?;
    let height_characters: Vec<RawCharacter> = get_characters_with_height(&raw_characters).unwrap();
    let no_height_characters: Vec<RawCharacter> =
        get_characters_with_no_height(&raw_characters).unwrap();
    print_to_file("output.json", &raw_characters);

    Ok(())
}

fn print_to_file(file_name: &str, data: &Vec<RawCharacter>) {
    let json = serde_json::to_string_pretty(&data).unwrap();
    let mut file = File::create(file_name).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

async fn get_characters(url: String) -> Result<Vec<RawCharacter>, Box<dyn std::error::Error>> {
    let mut result: Vec<RawCharacter> = Vec::new();
    let response: Response = reqwest::get(url).await?;
    let api_response: APIResponse = response.json().await?;
    result.extend(api_response.results);
    let mut response_next = api_response.next;
    while let Some(url) = &response_next {
        let response = reqwest::get(url).await?;
        let api_response: APIResponse = response.json().await?;
        result.extend(api_response.results);
        response_next = api_response.next;
    }
    Ok(result)
}

fn get_characters_with_height(
    characters: &[RawCharacter],
) -> Result<Vec<RawCharacter>, Box<dyn std::error::Error>> {
    let mut result: Vec<RawCharacter> = Vec::new();
    for character in characters.iter() {
        if let Some(height) = &character.height {
            if is_number(height.as_str()) {
                result.push(character.clone())
            }
        }
    }
    Ok(result)
}

fn get_characters_with_no_height(
    characters: &[RawCharacter],
) -> Result<Vec<RawCharacter>, Box<dyn std::error::Error>> {
    let mut result: Vec<RawCharacter> = Vec::new();
    for character in characters.iter() {
        if let Some(height) = &character.height {
            if !is_number(height.as_str()) {
                result.push(character.clone())
            }
        }
    }
    Ok(result)
}

fn is_number(s: &str) -> bool {
    s.parse::<i32>().is_ok()
}
