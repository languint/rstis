use yew::{function_component, html, Html};

#[function_component]
pub fn SplashScreen() -> Html {
    html! {
        <div class="splashscreen">
            <div class="splashscreen-text">
                <span class="splashscreen-inactive">{"          _   _     "}</span>
                <span class="splashscreen-inactive">{"         | | (_)    "}</span>
                <span class="splashscreen-inactive">{" _ __ ___| |_ _ ___ "}</span>
                <span class="splashscreen-inactive">{"| '__/ __| __| / __|"}</span>
                <span class="splashscreen-inactive">{"| |  \\__ \\ |_| \\__ \\"}</span>
                <span class="splashscreen-inactive">{"|_|  |___/\\__|_|___/"}</span>
            </div>
        </div>
    }
}
