use std::panic;

use gloo::events::EventListener;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{CustomEvent, CustomEventInit};
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub children: Children,
}

pub enum Msg {
    ShowError(String),
}

pub struct ErrorBoundary {
    error: Option<String>,
    _panic_listener: EventListener,
}

impl Component for ErrorBoundary {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic_info| {
            if let Some(window) = web_sys::window() {
                let event_init = CustomEventInit::new();
                let message = panic_info.to_string();
                event_init.set_detail(&message.into());
                if let Ok(event) = CustomEvent::new_with_event_init_dict("panic", &event_init) {
                    let _ = window.dispatch_event(&event);
                }
            }
            panic_hook(panic_info);
        }));

        let link = ctx.link().clone();
        let panic_listener =
            EventListener::new(&web_sys::window().unwrap(), "panic", move |event| {
                if let Some(event) = event.dyn_ref::<CustomEvent>() {
                    let detail = event.detail();
                    let msg = detail
                        .as_string()
                        .unwrap_or_else(|| "Unknown error".to_string());
                    link.send_message(Msg::ShowError(msg));
                }
            });

        Self {
            error: None,
            _panic_listener: panic_listener,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ShowError(msg) => {
                self.error = Some(msg);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(error) = &self.error {
            html! {
                <div class="panic-boundary">
                    <p>{ "A panic occurred:" }</p>
                    <pre>{ error }</pre>
                </div>
            }
        } else {
            html! { <>{ for ctx.props().children.iter() }</> }
        }
    }
}
