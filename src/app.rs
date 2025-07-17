use gloo::utils::document;
use gloo_timers::callback::Timeout;
use web_sys::{wasm_bindgen::JsCast, HtmlElement};
use yew::prelude::*;

use crate::components::{
    error_boundary::ErrorBoundary,
    splash_screen::{SplashScreen, SPLASHSCREEN_LINE_COUNT, SPLASHSCREEN_LINE_DURATION},
    title_screen::TitleScreen,
};

#[function_component(App)]
pub fn app() -> Html {
    let show_splash = use_state(|| true);
    let show_title = use_state(|| false);

    {
        let show_splash = show_splash.clone();
        let show_title = show_title.clone();
        use_effect_with((), move |_| {
            let timeout = Timeout::new(
                SPLASHSCREEN_LINE_DURATION * (SPLASHSCREEN_LINE_COUNT + 2),
                move || {
                    show_splash.set(false);
                    show_title.set(true);

                    let enter_debugger_button = document()
                        .query_selector("#enter-debugger-button")
                        .expect("Expected query selector `#enter-debugger-button` to succeed")
                        .expect("Expected `#enter-debugger-button` to exist in the DOM")
                        .dyn_into::<HtmlElement>()
                        .expect("Expected dyn_into::<HtmlElement>() to succeed");

                    enter_debugger_button
                        .focus()
                        .expect("Expected focus `#enter-debugger-button` to succeed");
                },
            );
            || {
                timeout.cancel();
            }
        });
    }

    html! {
        <ErrorBoundary>
            <div class="app">
                if *show_splash {
                    <SplashScreen />
                }
                <TitleScreen hidden={!*show_title}/>
            </div>
        </ErrorBoundary>
    }
}
