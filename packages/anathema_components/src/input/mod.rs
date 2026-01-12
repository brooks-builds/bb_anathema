use std::fs;

use anathema::{
    component::Component,
    prelude::ToSourceKind,
    state::{State, Value},
};

use crate::BBComponent;

pub struct BBInput {
    cursor_index: usize,
}

impl BBComponent for BBInput {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.prototype(
            Self::ident(),
            Self::load_template().to_template(),
            || Self { cursor_index: 0 },
            BBInputState::new,
        )
    }

    fn load_template() -> &'static str {
        include_str!("template.aml")
    }

    fn ident() -> &'static str {
        "BBInput"
    }
}

impl Component for BBInput {
    type State = BBInputState;

    type Message = ();

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if let Some(initial_value) = context.attribute("initial_value") {
            let value = initial_value.as_str().unwrap_or_default();
            let cursor_index = value.len();

            state.value.set(value.to_owned());
            state.cursor_index.set(cursor_index);

            self.cursor_index = cursor_index;
        }
    }
}

#[derive(Debug, State, Default)]
pub struct BBInputState {
    value: Value<String>,
    cursor_index: Value<usize>,
}

impl BBInputState {
    pub fn new() -> Self {
        Self::default()
    }
}
