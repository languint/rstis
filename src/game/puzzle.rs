use serde::Deserialize;

use crate::game::node::Node;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub enum PuzzleStatus {
    InProgress,
    Completed,
}

impl ToString for PuzzleStatus {
    fn to_string(&self) -> String {
        match self {
            PuzzleStatus::InProgress => "IN PROGRESS".to_string(),
            PuzzleStatus::Completed => "COMPLETED".to_string(),
        }
    }
}

impl PuzzleStatus {
    pub fn get_class(&self) -> String {
        match self {
            PuzzleStatus::InProgress => "in-progress".to_string(),
            PuzzleStatus::Completed => "completed".to_string(),
        }
    }
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct PuzzleTest {
    pub label: String,
    pub expected: Vec<i32>,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct PuzzleMeta {
    pub title: String,
    pub description: String,
    pub status: PuzzleStatus,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct PuzzleData {
    pub tests: Vec<PuzzleTest>,
    pub nodes: Vec<Node>,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Puzzle {
    pub meta: PuzzleMeta,
    pub data: PuzzleData,
}

impl Puzzle {
    pub fn load_from_toml(toml_str: &str) -> Result<Self, String> {
        match toml::from_str(toml_str) {
            Ok(puzzle) => Ok(puzzle),
            Err(e) => Err(format!("Failed to parse TOML: {}", e)),
        }
    }
}
