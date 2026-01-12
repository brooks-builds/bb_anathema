use crate::BBComponent;
use anathema::{
    component::Component,
    prelude::ToSourceKind,
    state::{State, Value},
};

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
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if let Some(initial_value) = context.attribute("initial_value") {
            let value = initial_value.as_str().unwrap_or_default();
            let cursor_index = value.len();

            state.value.set(value.to_owned());
            state.cursor_index.set(cursor_index);

            self.cursor_index = cursor_index;
        }
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match key.code {
            anathema::component::KeyCode::Char(character) => {
                let mut value = state.value.to_mut();
                let mut index = state.cursor_index.to_mut();

                value.insert(*index, character);
                *index += 1;

                context.publish("on_change", value.clone());
            }
            anathema::component::KeyCode::Backspace => {
                let mut value = state.value.to_mut();
                let mut index = state.cursor_index.to_mut();

                if *index == 0 {
                    return;
                }

                value.remove(*index - 1);
                *index -= 1;

                context.publish("on_change", value.clone());
            }
            anathema::component::KeyCode::Left => {
                let mut cursor_index = state.cursor_index.to_mut();

                if *cursor_index > 0 {
                    *cursor_index -= 1;
                }
            }
            anathema::component::KeyCode::Right => {
                let mut cursor_index = state.cursor_index.to_mut();
                let value = state.value.to_ref();

                if *cursor_index < value.len() {
                    *cursor_index += 1;
                }
            }
            anathema::component::KeyCode::Up => {
                state.cursor_index.set(0);
            }
            anathema::component::KeyCode::Down => {
                let value = state.value.to_ref();

                state.cursor_index.set(value.len());
            }
            anathema::component::KeyCode::Home => state.cursor_index.set(0),
            anathema::component::KeyCode::End => {
                let value = state.value.to_ref();

                state.cursor_index.set(value.len());
            }
            anathema::component::KeyCode::Delete => {
                let mut value = state.value.to_mut();
                let index = *state.cursor_index.to_ref();

                if index >= value.len() {
                    return;
                }

                value.remove(index);

                context.publish("on_change", value.clone());
            }
            _ => (),
        }
    }

    fn accept_focus(&self) -> bool {
        true
    }
}

#[derive(Debug, State, Default)]
pub struct BBInputState {
    value: Value<String>,
    cursor_index: Value<usize>,
}

impl BBInputState {
    pub fn new() -> Self {
        let value = Value::default();
        let cursor_index = Value::default();

        Self {
            value,
            cursor_index,
        }
    }
}
