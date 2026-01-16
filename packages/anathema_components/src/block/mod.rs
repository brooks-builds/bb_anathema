use crate::InteractiveState;
use anathema::{
    component::{Component, KeyCode},
    default_widgets::Overflow,
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

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        _state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let code = key.code;

        children
            .elements()
            .by_tag("overflow")
            .first(|element, _overflow_attributes| {
                let Some(overflow) = element.try_to::<Overflow>() else {
                    return;
                };

                match code {
                    KeyCode::Down => {
                        overflow.scroll_down();
                    }
                    KeyCode::Up => overflow.scroll_up(),
                    KeyCode::Char('k') => overflow.scroll_up(),
                    KeyCode::Char('j') => overflow.scroll_down(),
                    _ => (),
                }
            });
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if event.name() != "copy" {
            return;
        }
        let Some(value) = context.attribute("value") else {
            return;
        };
        let Some(value) = value.as_str() else { return };
        if let Ok(mut clipboard) = Clipboard::new() {
            clipboard.set_text(value).ok();
            state.copy_label.set("copied".to_owned());

            context.publish("copied", ());
        }
    }

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state
            .interactive_state
            .set(String::from(InteractiveState::Focused));
    }

    fn on_blur(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state
            .interactive_state
            .set(String::from(InteractiveState::Normal));
    }
}

#[derive(Debug, State)]
pub struct BBBlockState {
    width: Value<u16>,
    height: Value<u16>,
    copy_label: Value<String>,
    interactive_state: Value<String>,
}

impl Default for BBBlockState {
    fn default() -> Self {
        let width = Value::default();
        let height = Value::default();
        let copy_label = Value::new("copy".to_owned());
        let interactive_state = Value::new(String::from(InteractiveState::Normal));

        Self {
            width,
            height,
            copy_label,
            interactive_state,
        }
    }
}
