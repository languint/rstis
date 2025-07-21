use yew::prelude::*;

use crate::game::puzzle::Puzzle;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct GameState {
    pub current_puzzle: Option<Puzzle>,
    pub current_node_id: Option<usize>,
    pub loaded_puzzles: Vec<Puzzle>,
    pub is_running: bool,
}

#[derive(Clone, PartialEq)]
pub struct GameStateContext {
    pub state: UseStateHandle<GameState>,
}

#[allow(dead_code)]
impl GameStateContext {
    pub fn set_current_puzzle(&self, puzzle: Option<Puzzle>) {
        self.state.set(GameState {
            current_puzzle: puzzle,
            ..(*self.state).clone()
        });
    }
    pub fn set_current_node_id(&self, node_id: Option<usize>) {
        self.state.set(GameState {
            current_node_id: node_id,
            ..(*self.state).clone()
        });
    }
    pub fn set_is_running(&self, is_running: bool) {
        self.state.set(GameState {
            is_running,
            ..(*self.state).clone()
        });
    }
    pub fn add_loaded_puzzle(&self, puzzle: Puzzle) {
        let mut state = (*self.state).clone();
        state.loaded_puzzles.push(puzzle);
        self.state.set(state);
    }
    pub fn clear_loaded_puzzles(&self) {
        self.state.set(GameState {
            loaded_puzzles: Vec::new(),
            ..(*self.state).clone()
        });
    }
}

#[hook]
pub fn use_game_state_context() -> GameStateContext {
    let state = use_context::<UseStateHandle<GameState>>().expect("GameStateContext not found");
    GameStateContext { state }
}
