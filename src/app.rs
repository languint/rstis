use yew::prelude::*;

use crate::components::{
    error_boundary::ErrorBoundary,
    scenario_wall::ScenarioWall,
    splash_screen::{SplashScreen},
    title_screen::TitleScreen,
    open_elements_context::OpenElements,
};

#[function_component(App)]
pub fn app() -> Html {
    let open_elements = use_state(|| OpenElements {
        show_splash: true,
        show_title: false,
        show_scenario_wall: false,
    });

    html! {
        <ContextProvider<UseStateHandle<OpenElements>> context={open_elements.clone()}>
            <InnerApp />
        </ContextProvider<UseStateHandle<OpenElements>>>
    }
}

#[function_component(InnerApp)]
fn inner_app() -> Html {
    let open_ctx = use_context::<UseStateHandle<OpenElements>>().expect("OpenElements context missing");

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
