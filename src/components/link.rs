use std::error::Error;

use anathema::{
    backend::tui::TuiBackend,
    component::{Component, ComponentId},
    runtime::RuntimeBuilder,
    state::{State, Value},
};

pub struct Link;

impl Link {
    pub fn register(
        runtime: &mut RuntimeBuilder<TuiBackend>,
        value: impl Into<String>,
        href: impl Into<String>,
    ) -> Result<ComponentId<()>, impl Error> {
        runtime.register_component(
            "bb_link",
            "templates/link.aml",
            Self,
            LinkState::new(value, href),
        )
    }
}

impl Component for Link {
    type State = LinkState;

    type Message = ();
}

#[derive(State)]
pub struct LinkState {
    pub value: Value<String>,
    pub href: Value<String>,
}

impl LinkState {
    pub fn new(value: impl Into<String>, href: impl Into<String>) -> Self {
        Self {
            value: Value::new(value.into()),
            href: Value::new(href.into()),
        }
    }
}
