use yew::{function_component, html, Callback, Html};
use crate::components::open_elements_context::use_open_elements_context;

#[function_component]
pub fn TitleScreen() -> Html {
    let open_ctx = use_open_elements_context();
    let hidden = !open_ctx.state.show_title;
    let onclick = {
        let open_ctx = open_ctx.clone();
        Callback::from(move |_| {
            open_ctx.state.set(crate::components::open_elements_context::OpenElements {
                show_splash: false,
                show_title: false,
                show_scenario_wall: true,
            });
        })
    };

    html! {
        <div class={format!("title-screen {}", if hidden {"hidden" } else {""})}>
            <div class="splashscreen-text">
                <span class={"splashscreen-active" }>{"          _   _     "}</span>
                <span class={"splashscreen-active" }>{"         | | (_)    "}</span>
                <span class={"splashscreen-active" }>{" _ __ ___| |_ _ ___ "}</span>
                <span class={"splashscreen-active"}>{"| '__/ __| __| / __|"}</span>
                <span class={"splashscreen-active"} >{"| |  \\__ \\ |_| \\__ \\"}</span>
                <span class={"splashscreen-active"}>{"|_|  |___/\\__|_|___/"}</span>
            </div>
            <button tabindex="0" id="enter-debugger-button" {onclick}>{"Enter Debugger"}<span class="material-symbols-outlined">{"keyboard_return"}</span>
            </button>
        </div>
    }
}
