use yew::prelude::*;

#[derive(Clone, PartialEq, Debug, Default)]
pub struct OpenElements {
    pub show_splash: bool,
    pub show_title: bool,
    pub show_scenario_wall: bool,
}

#[derive(Clone, PartialEq)]
pub struct OpenElementsContext {
    pub state: UseStateHandle<OpenElements>,
}

#[allow(dead_code)]
impl OpenElementsContext {
    pub fn set_show_splash(&self, value: bool) {
        self.state.set(OpenElements {
            show_splash: value,
            ..(*self.state).clone()
        });
    }
    pub fn set_show_title(&self, value: bool) {
        self.state.set(OpenElements {
            show_title: value,
            ..(*self.state).clone()
        });
    }
    pub fn set_show_scenario_wall(&self, value: bool) {
        self.state.set(OpenElements {
            show_scenario_wall: value,
            ..(*self.state).clone()
        });
    }
}

#[allow(dead_code)]
pub type OpenElementsContextProvider = UseReducerHandle<OpenElements>;

#[hook]
pub fn use_open_elements_context() -> OpenElementsContext {
    let state =
        use_context::<UseStateHandle<OpenElements>>().expect("OpenElementsContext not found");
    OpenElementsContext { state }
}
