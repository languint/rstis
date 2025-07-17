use gloo_timers::callback::Timeout;
use yew::{function_component, html, use_effect_with, use_state, Html};

pub const SPLASHSCREEN_LINE_DURATION: u32 = 300;
pub const SPLASHSCREEN_LINE_COUNT: u32 = 6;

#[function_component]
pub fn SplashScreen() -> Html {
    let active_lines = use_state(|| 0);
    let is_done = use_state(|| false);

    {
        let active_lines = active_lines.clone();
        let is_done = is_done.clone();
        use_effect_with((), move |_| {
            let mut timeouts = Vec::new();

            for i in 1..=SPLASHSCREEN_LINE_COUNT {
                let active_lines = active_lines.clone();
                let timeout = Timeout::new((i * SPLASHSCREEN_LINE_DURATION) as u32, move || {
                    active_lines.set(i);
                });
                timeouts.push(timeout);
            }

            let is_done = is_done.clone();
            let done_timeout = Timeout::new(
                ((SPLASHSCREEN_LINE_COUNT + 2) * SPLASHSCREEN_LINE_DURATION) as u32,
                move || {
                    is_done.set(true);
                },
            );
            timeouts.push(done_timeout);

            move || {
                for timeout in timeouts {
                    timeout.cancel();
                }
            }
        });
    }

    html! {
        <div class={if *is_done {"splashscreen hidden"} else {"splashscreen"}}>
            <div class="splashscreen-text">
                <span class={if *active_lines >= 1 {"splashscreen-active"} else {"splashscreen-inactive"}}>{"          _   _     "}</span>
                <span class={if *active_lines >= 2 {"splashscreen-active"} else {"splashscreen-inactive"}}>{"         | | (_)    "}</span>
                <span class={if *active_lines >= 3 {"splashscreen-active"} else {"splashscreen-inactive"}}>{" _ __ ___| |_ _ ___ "}</span>
                <span class={if *active_lines >= 4 {"splashscreen-active"} else {"splashscreen-inactive"}}>{"| '__/ __| __| / __|"}</span>
                <span class={if *active_lines >= 5 {"splashscreen-active"} else {"splashscreen-inactive"}}>{"| |  \\__ \\ |_| \\__ \\"}</span>
                <span class={if *active_lines >= 6 {"splashscreen-active"} else {"splashscreen-inactive"}}>{"|_|  |___/\\__|_|___/"}</span>
            </div>
        </div>
    }
}
