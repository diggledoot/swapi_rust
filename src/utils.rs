use std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
};

use reqwest::Response;

use crate::models::{APIResponse, OutputCharacter, RawCharacter};

pub fn sort_by_gender(
    raw_characters: &[RawCharacter],
    gender_string: &str,
) -> Result<Vec<RawCharacter>, Box<dyn std::error::Error>> {
    let result: Vec<RawCharacter> = raw_characters
        .iter()
        .filter(|character| character.gender.as_ref().unwrap() == gender_string)
        .cloned()
        .collect();
    Ok(result)
}

pub fn extract_gender(
    characters: &[RawCharacter],
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = characters
        .iter()
        .filter_map(|character| character.gender.clone())
        .collect::<HashSet<String>>()
        .into_iter()
        .collect();
    result.sort();
    Ok(result)
}

pub fn write_to_file(file_name: &str, data: &[OutputCharacter]) {
    let json: String = serde_json::to_string_pretty(&data)
        .unwrap_or_else(|_err| panic!("Failed to convert data to String!"));
    let file: File =
        File::create(file_name).unwrap_or_else(|_err| panic!("Failed to create file!"));
    let mut writer = BufWriter::new(file);
    writer
        .write_all(json.as_bytes())
        .unwrap_or_else(|_err| panic!("Failed to write to file!"));
}

pub async fn get_characters(url: String) -> Result<Vec<RawCharacter>, Box<dyn std::error::Error>> {
    let mut result: Vec<RawCharacter> = Vec::new();
    let response: Response = reqwest::get(url).await?;
    let api_response: APIResponse = response.json().await?;
    result.extend(api_response.results);
    let mut response_next: Option<String> = api_response.next;
    while let Some(url) = &response_next {
        let response: Response = reqwest::get(url).await?;
        let api_response: APIResponse = response.json().await?;
        result.extend(api_response.results);
        response_next = api_response.next;
    }
    Ok(result)
}

pub fn get_characters_with_height(
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

pub fn get_characters_with_no_height(
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

fn is_number(string: &str) -> bool {
    string.parse::<i32>().is_ok()
}
