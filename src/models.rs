#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct RawCharacter {
    pub name: Option<String>,
    pub height: Option<String>,
    pub gender: Option<String>,
    pub birth_year: Option<String>,
    pub eye_color: Option<String>,
    pub hair_color: Option<String>,
    pub mass: Option<String>,
    pub skin_color: Option<String>,
    pub homeworld: Option<String>,
    pub films: Vec<Option<String>>,
    pub species: Vec<Option<String>>,
    pub starships: Vec<Option<String>>,
    pub vehicles: Vec<Option<String>>,
    pub url: Option<String>,
    pub created: Option<String>,
    pub edited: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct OutputCharacter {
    pub gender: String,
    pub characters: Vec<RawCharacter>,
}

pub struct Character {
    pub name: Option<String>,
    pub height: i32,
    pub gender: Option<String>,
}
#[derive(Debug, Deserialize, Clone, Serialize)]
pub struct APIResponse {
    pub count: i32,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<RawCharacter>,
}
