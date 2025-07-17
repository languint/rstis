use crate::components::open_elements_context::use_open_elements_context;
use gloo_timers::callback::Timeout;
use yew::{function_component, html, use_effect_with, use_state, Html};

pub const SPLASHSCREEN_LINE_DURATION: u32 = 300;
pub const SPLASHSCREEN_LINE_COUNT: u32 = 6;

#[function_component]
pub fn SplashScreen() -> Html {
    let active_lines = use_state(|| 0);
    let open_ctx = use_open_elements_context();

    {
        let active_lines = active_lines.clone();
        let open_ctx = open_ctx.clone();
        use_effect_with((), move |_| {
            let mut timeouts = Vec::new();

            for i in 1..=SPLASHSCREEN_LINE_COUNT {
                let active_lines = active_lines.clone();
                let timeout = Timeout::new((i * SPLASHSCREEN_LINE_DURATION) as u32, move || {
                    active_lines.set(i);
                });
                timeouts.push(timeout);
            }

            let open_ctx = open_ctx.clone();
            let done_timeout = Timeout::new(
                ((SPLASHSCREEN_LINE_COUNT + 2) * SPLASHSCREEN_LINE_DURATION) as u32,
                move || {
                    open_ctx
                        .state
                        .set(crate::components::open_elements_context::OpenElements {
                            show_splash: false,
                            show_title: true,
                            show_scenario_wall: false,
                        });
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

    let hidden = !open_ctx.state.show_splash;

    html! {
        <div class={if hidden {"splashscreen hidden"} else {"splashscreen"}}>
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
