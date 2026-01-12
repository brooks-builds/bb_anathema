use anathema::{
    component::Component,
    state::{State, Value},
};
use arboard::Clipboard;
use bb_anathema_macros::BBComponent;

#[derive(BBComponent)]
#[bb_component(state = BBBlockState)]
pub struct BBBlock;

impl Component for BBBlock {
    type State = BBBlockState;

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if context.attribute("copy").is_none()
            || context
                .attribute("copy")
                .is_some_and(|copy| !copy.as_bool().unwrap_or_default())
        {
            return;
        }

        children
            .elements()
            .at_position(mouse.pos())
            .first(|_el, attributes| {
                if mouse.left_down() {
                    state.foreground.set("red".to_owned());
                    state.started_clicking.set(true);
                } else if mouse.left_up() && *state.started_clicking.to_ref() {
                    state.foreground.set("white".to_owned());
                    state.started_clicking.set(false);
                    // think this is a bug, the attributes are never updated
                    let Some(value) = attributes.get("value") else {
                        return;
                    };
                    let Some(value) = value.as_str() else { return };
                    let Ok(mut clipboard) = Clipboard::new() else {
                        return;
                    };

                    clipboard.set_text(value).ok();
                }
            });
    }
}

#[derive(Debug, State, Default)]
pub struct BBBlockState {
    started_clicking: Value<bool>,
    foreground: Value<String>,
}

impl BBBlockState {
    pub fn new() -> Self {
        let started_clicking = Value::default();
        let foreground = Value::new(String::from("white"));

        Self {
            started_clicking,
            foreground,
        }
    }
}
