use crate::{
    context::{
        game_state_context::use_game_state_context,
        open_elements_context::use_open_elements_context,
    },
    game::puzzle::Puzzle,
};
use yew::{function_component, html, Html, Properties};

#[function_component]
pub fn ScenarioWall() -> Html {
    let open_ctx = use_open_elements_context();
    let game_state_ctx = use_game_state_context();

    let hidden = !open_ctx.state.show_scenario_wall;
    html! {
        <div class={if hidden {"scenario-wall hidden"} else {"scenario-wall"}}>
            <div class="scenario-wall-meta">
                <h1>{"Puzzles"}</h1>
                <p>{"Select a puzzle to start solving."}</p>
            </div>
            <div class="scenario-wall-grid">
                { game_state_ctx.state.loaded_puzzles.iter().map(|p| html! {<ScenarioWallPuzzle puzzle={p.clone()}/>}).collect::<Html>() }
            </div>
        </div>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ScenarioWallPuzzleProps {
    pub puzzle: Puzzle,
}

#[function_component]
pub fn ScenarioWallPuzzle(props: &ScenarioWallPuzzleProps) -> Html {
    let puzzle = &props.puzzle;
    html! {
        <div class="scenario-wall-puzzle">
            <h3>{&puzzle.meta.title}</h3>
            <p>{&puzzle.meta.description}</p>
            <p class={format!("puzzle-status {}", &puzzle.meta.status.get_class())}>{&puzzle.meta.status.to_string()}</p>
        </div>
    }
}
