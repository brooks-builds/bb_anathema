use anathema::{
    component::{Component, Context},
    state::{State, Value},
};
use bb_anathema_macros::BBComponent;

use crate::InteractiveState;

#[derive(BBComponent)]
#[bb_component(state = BBCheckboxState)]
pub struct BBCheckbox;

impl Component for BBCheckbox {
    type State = BBCheckboxState;

    type Message = ();

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let is_mouse_over = children
            .elements()
            .at_position(mouse.pos())
            .first(|_, _| {})
            .is_some();
        let interactive_state = InteractiveState::from(state.interactive_state.to_ref().as_str());
        let mut checked = state.checked.to_mut();

        if is_mouse_over && mouse.left_down() {
            state
                .interactive_state
                .set(String::from(InteractiveState::MouseDown));
        } else if is_mouse_over && mouse.left_up() && interactive_state.is_mouse_down() {
            *checked = !*checked;
            publish_checked(&mut context, *checked);
        } else if is_mouse_over {
            state
                .interactive_state
                .set(String::from(InteractiveState::MouseOver));
        } else {
            state
                .interactive_state
                .set(String::from(InteractiveState::Normal));
        }
    }

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let checked = context
            .attribute("checked")
            .unwrap_or(&anathema::resolver::ValueKind::Bool(false));
        let checked = checked.as_bool().unwrap_or_default();

        state.checked.set(checked);
    }

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: Context<'_, '_, Self::State>,
    ) {
        state
            .interactive_state
            .set(String::from(InteractiveState::Focused));
    }

    fn on_blur(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: Context<'_, '_, Self::State>,
    ) {
        state
            .interactive_state
            .set(String::from(InteractiveState::Normal));
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: Context<'_, '_, Self::State>,
    ) {
        if matches!(key.code, anathema::component::KeyCode::Enter) {
            let mut checked = state.checked.to_mut();

            *checked = !*checked;
            publish_checked(&mut context, *checked);
        }
    }
}

#[derive(Debug, State)]
pub struct BBCheckboxState {
    checked: Value<bool>,
    interactive_state: Value<String>,
}

impl Default for BBCheckboxState {
    fn default() -> Self {
        let checked = Value::default();
        let interactive_state = Value::new(String::from(InteractiveState::Normal));

        Self {
            checked,
            interactive_state,
        }
    }
}

fn publish_checked(context: &mut Context<'_, '_, BBCheckboxState>, checked_state: bool) {
    context.publish("on_change", checked_state);
}
