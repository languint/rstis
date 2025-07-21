use yew::prelude::*;

use crate::components::{
    error_boundary::ErrorBoundary, scenario_wall::ScenarioWall, splash_screen::SplashScreen,
    title_screen::TitleScreen,
};

use crate::context::game_state_context::GameState;
use crate::context::open_elements_context::OpenElements;
use crate::game::puzzle::Puzzle;

#[function_component(App)]
pub fn app() -> Html {
    let open_elements = use_state(|| OpenElements {
        show_splash: true,
        show_title: false,
        show_scenario_wall: false,
    });

    let game_state = use_state(|| GameState {
        current_puzzle: None,
        loaded_puzzles: Vec::new(),
        current_node_id: None,
        is_running: false,
    });

    {
        let game_state = game_state.clone();

        use_effect_with((), move |_| {
            let mut loaded_puzzles = Vec::new();
            let puzzle_files = [
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
                include_str!("../puzzles/test_puzzle.toml"),
            ];
            for toml_str in puzzle_files.iter() {
                match Puzzle::load_from_toml(toml_str) {
                    Ok(puzzle) => loaded_puzzles.push(puzzle),
                    Err(e) => gloo_console::error!(format!("Failed to load puzzle: {}", e)),
                }
            }
            game_state.set(GameState {
                loaded_puzzles,
                ..(*game_state).clone()
            });
            || ()
        });
    }

    html! {
        <ContextProvider<UseStateHandle<OpenElements>> context={open_elements.clone()}>
            <ContextProvider<UseStateHandle<GameState>> context={game_state.clone()}>
                <InnerApp />
            </ContextProvider<UseStateHandle<GameState>>>
        </ContextProvider<UseStateHandle<OpenElements>>>
    }
}

#[function_component(InnerApp)]
fn inner_app() -> Html {
    let open_ctx =
        use_context::<UseStateHandle<OpenElements>>().expect("OpenElements context missing");

    let game_state_ctx =
        use_context::<UseStateHandle<GameState>>().expect("GameState context missing");

    html! {
        <ErrorBoundary>
            <div class="app">
                { if open_ctx.show_splash { html! { <SplashScreen /> } } else { html! {} } }
                <ScenarioWall />
                <TitleScreen />
            </div>
        </ErrorBoundary>
    }
}
