#![allow(unused)]

use std::{fs::File, io::Write};

use reqwest::Response;
use serde_json::Error;
use swapi::models::{APIResponse, OutputCharacter, RawCharacter};

const SWAPI_PEOPLE_URL: &str = "https://swapi.dev/api/people";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_characters: Vec<RawCharacter> = get_characters(SWAPI_PEOPLE_URL.to_string()).await?;
    let genders: Vec<String> = extract_gender(&raw_characters).unwrap_or_else(|err| {
        panic!("Failed to extract gender string!");
    });
    let mut output_characters: Vec<OutputCharacter> = Vec::new();
    for gender in genders.iter() {
        let gendered_characters = sort_by_gender(&raw_characters, gender)
            .unwrap_or_else(|err| panic!("Failed to sort gender!"));
        let output_character: OutputCharacter = OutputCharacter {
            gender: String::from(gender),
            characters: gendered_characters,
        };
        output_characters.push(output_character);
        // let height_characters: Vec<RawCharacter> = get_characters_with_height(&raw_characters)
        //     .unwrap_or_else(|err| {
        //         panic!("Failed to extract characters with height!");
        //     });
        // let no_height_characters: Vec<RawCharacter> =
        //     get_characters_with_no_height(&raw_characters).unwrap_or_else(|err| {
        //         panic!("Failed to extract characters with no height!");
        //     });
    }

    write_to_file("output.json", &output_characters);

    Ok(())
}

fn sort_by_gender(
    raw_characters: &[RawCharacter],
    gender_string: &String,
) -> Result<Vec<RawCharacter>, Box<dyn std::error::Error>> {
    let mut result = Vec::new();
    for character in raw_characters.iter() {
        if let Some(gender) = &character.gender {
            if gender.eq(gender_string) {
                result.push(character.clone());
            }
        }
    }
    Ok(result)
}

fn extract_gender(characters: &[RawCharacter]) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut result: Vec<String> = Vec::new();
    for character in characters.iter() {
        if let Some(gender) = &character.gender {
            if !result.contains(gender) {
                result.push(gender.clone());
            }
        }
    }
    Ok(result)
}

fn write_to_file(file_name: &str, data: &[OutputCharacter]) {
    let json: String = serde_json::to_string_pretty(&data)
        .unwrap_or_else(|err| panic!("Failed to convert data to String!"));
    let mut file: File =
        File::create(file_name).unwrap_or_else(|err| panic!("Failed to create file!"));
    file.write_all(json.as_bytes())
        .unwrap_or_else(|err| panic!("Failed to write to file!"));
}

async fn get_characters(url: String) -> Result<Vec<RawCharacter>, Box<dyn std::error::Error>> {
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

fn is_number(string: &str) -> bool {
    string.parse::<i32>().is_ok()
}
