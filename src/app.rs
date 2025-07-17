use yew::prelude::*;

use crate::components::splashscreen::SplashScreen;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <div class="app">
            <SplashScreen />
        </div>
    }
}
