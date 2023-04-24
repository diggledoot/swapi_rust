#![allow(unused)]

use std::{fs::File, io::Write};

use reqwest::Response;
use serde_json::Error;
use swapi::{
    models::{APIResponse, Character, OutputCharacter, RawCharacter},
    utils::{
        extract_gender, get_characters, get_characters_with_height, get_characters_with_no_height,
        sort_by_gender, write_to_file,
    },
};

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

        let mut height_characters: Vec<RawCharacter> =
            get_characters_with_height(&gendered_characters).unwrap_or_else(|err| {
                panic!("Failed to extract characters with height!");
            });

        height_characters.sort_by(|a, b| {
            let lhs: i32 = a.height.as_ref().unwrap().parse::<i32>().unwrap();
            let rhs: i32 = b.height.as_ref().unwrap().parse::<i32>().unwrap();
            lhs.cmp(&rhs)
        });

        let mut no_height_characters: Vec<RawCharacter> =
            get_characters_with_no_height(&gendered_characters).unwrap_or_else(|err| {
                panic!("Failed to extract characters with no height!");
            });

        no_height_characters.sort_by(|a, b| {
            let lhs = a.name.as_ref().unwrap();
            let rhs = b.name.as_ref().unwrap();
            lhs.cmp(rhs)
        });

        height_characters.extend(no_height_characters);

        let mut concatenated_characters: Vec<Character> = Vec::new();

        for character in height_characters.iter() {
            let name: String = character.name.clone().unwrap();
            let height: String = character.height.clone().unwrap();
            let item: Character = Character {
                name: Some(name),
                height: Some(height),
            };
            concatenated_characters.push(item);
        }

        let output_character: OutputCharacter = OutputCharacter {
            gender: String::from(gender),
            characters: concatenated_characters,
        };
        output_characters.push(output_character);
    }

    write_to_file("output.json", &output_characters);

    Ok(())
}
