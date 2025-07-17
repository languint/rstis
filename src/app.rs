use gloo_timers::callback::Timeout;
use yew::prelude::*;

use crate::components::splashscreen::{
    SplashScreen, SPLASHSCREEN_LINE_COUNT, SPLASHSCREEN_LINE_DURATION,
};

#[function_component(App)]
pub fn app() -> Html {
    let show_splash = use_state(|| true);

    {
        let show_splash = show_splash.clone();
        use_effect_with((), move |_| {
            let timeout = Timeout::new(
                SPLASHSCREEN_LINE_DURATION * (SPLASHSCREEN_LINE_COUNT + 2),
                move || {
                    show_splash.set(false);
                },
            );
            || {
                timeout.cancel();
            }
        });
    }

    html! {
        <div class="app">
            if *show_splash {
                <SplashScreen />
            } else {
                <div class="main-content">
                    <h1>{ "Welcome to the App!" }</h1>
                </div>
            }
        </div>
    }
}
