use gloo_timers::callback::Timeout;
use yew::{function_component, html, use_effect_with, use_state, Html};

#[function_component]
pub fn TitleScreen() -> Html {
    html! {
        <div class={"title-screen"}>
            <div class="splashscreen-text">
                <span class={"splashscreen-active" }>{"          _   _     "}</span>
                <span class={"splashscreen-active" }>{"         | | (_)    "}</span>
                <span class={"splashscreen-active" }>{" _ __ ___| |_ _ ___ "}</span>
                <span class={"splashscreen-active"}>{"| '__/ __| __| / __|"}</span>
                <span class={"splashscreen-active"} >{"| |  \\__ \\ |_| \\__ \\"}</span>
                <span class={"splashscreen-active"}>{"|_|  |___/\\__|_|___/"}</span>
            </div>
            <button>{"Enter Debugger"}</button>
        </div>
    }
}
