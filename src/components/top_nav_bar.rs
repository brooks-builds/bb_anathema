use std::error::Error;

use anathema::{
    backend::tui::TuiBackend,
    component::{Component, ComponentId},
    runtime::RuntimeBuilder,
    state::{State, Value},
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

    fn resize(
        &mut self,
        state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        context: anathema::prelude::Context<'_>,
    ) {
        let width = context.viewport.size().width;

        state.width.set(width);
    }

    fn tick(
        &mut self,
        state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        context: anathema::prelude::Context<'_>,
        _dt: std::time::Duration,
    ) {
        let width = context.viewport.size().width;

        state.width.set(width);
    }

    fn accept_focus(&self) -> bool {
        false
    }
}

#[derive(State)]
pub struct TopNavBarState {
    pub width: Value<usize>,
}

impl TopNavBarState {
    pub fn new() -> Self {
        let width = Value::new(30);

        Self { width }
    }
}
