use std::error::Error;

use anathema::{
    backend::tui::TuiBackend,
    component::{Component, ComponentId},
    runtime::RuntimeBuilder,
    state::State,
};

pub struct TopNavBar;

impl TopNavBar {
    pub fn new() -> Self {
        Self
    }

    pub fn register(
        runtime: &mut RuntimeBuilder<TuiBackend>,
    ) -> Result<ComponentId<()>, impl Error> {
        runtime.register_component(
            "bb_top_nav_bar",
            "templates/top_nav_bar.aml",
            Self::new(),
            TopNavBarState::new(),
        )
    }
}

impl Component for TopNavBar {
    type State = TopNavBarState;

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }
}

#[derive(State)]
pub struct TopNavBarState {}

impl TopNavBarState {
    pub fn new() -> Self {
        Self {}
    }
}
