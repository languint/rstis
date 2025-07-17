use gloo_timers::callback::Timeout;
use yew::prelude::*;

use crate::components::{splash_screen::{
    SplashScreen, SPLASHSCREEN_LINE_COUNT, SPLASHSCREEN_LINE_DURATION,
}, title_screen::TitleScreen};

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
            } else if *show_title {
                <TitleScreen />
            }
        </div>
    }
}
