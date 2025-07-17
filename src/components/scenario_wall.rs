use yew::{function_component, html, Html};
use crate::components::open_elements_context::use_open_elements_context;

#[function_component]
pub fn ScenarioWall() -> Html {
    let open_ctx = use_open_elements_context();
    let hidden = !open_ctx.state.show_scenario_wall;
    html! {
        <div class={if hidden {"scenario-wall hidden"} else {"scenario-wall"}}>
            <div class="scenario-wall-grid">
                { "" }
            </div>
        </div>
    }
}
