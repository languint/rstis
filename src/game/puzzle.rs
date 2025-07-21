use serde::Deserialize;

use crate::game::node::{Node, NodeType};

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
pub struct PuzzleInput {
    pub label: String,
    pub input: Vec<i32>,
    pub into_node_id: usize,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct PuzzleOutput {
    pub label: String,
    pub output: Vec<i32>,
    pub from_node_id: usize,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct PuzzleMeta {
    pub title: String,
    pub description: String,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct PuzzleData {
    pub inputs: Vec<PuzzleInput>,
    pub outputs: Vec<PuzzleOutput>,
    pub node_types: Vec<NodeType>,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Puzzle {
    pub meta: PuzzleMeta,
    pub data: PuzzleData,
}

impl Puzzle {
    pub fn load_from_json(json_str: &str) -> Result<Self, String> {
        match serde_json::from_str(json_str) {
            Ok(puzzle) => Ok(puzzle),
            Err(e) => Err(format!("Failed to parse JSON: {}", e)),
        }
    }

    pub fn get_local_storage_status(&self) -> PuzzleStatus {
        PuzzleStatus::InProgress
    }
}
