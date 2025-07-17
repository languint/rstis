use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub hidden: bool,
}

#[function_component]
pub fn TitleScreen(props: &Props) -> Html {
    html! {
        <div class={format!("title-screen {}", if props.hidden {"hidden" } else {""})}>
            <div class="splashscreen-text">
                <span class={"splashscreen-active" }>{"          _   _     "}</span>
                <span class={"splashscreen-active" }>{"         | | (_)    "}</span>
                <span class={"splashscreen-active" }>{" _ __ ___| |_ _ ___ "}</span>
                <span class={"splashscreen-active"}>{"| '__/ __| __| / __|"}</span>
                <span class={"splashscreen-active"} >{"| |  \\__ \\ |_| \\__ \\"}</span>
                <span class={"splashscreen-active"}>{"|_|  |___/\\__|_|___/"}</span>
            </div>
            <button tabindex="0" id="enter-debugger-button">{"Enter Debugger"}<span class="material-symbols-outlined">{"keyboard_return"}</span>
            </button>
        </div>
    }
}
