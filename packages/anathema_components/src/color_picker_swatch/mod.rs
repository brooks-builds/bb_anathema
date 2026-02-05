use anathema::{
    component::{Component, KeyCode},
    state::{State, Value},
};
use bb_anathema_macros::BBComponent;

use crate::InteractiveState;

#[derive(BBComponent)]
#[bb_component(state = BBColorPickerSwatchState)]
pub struct BBColorPickerSwatch;

impl Component for BBColorPickerSwatch {
    type State = BBColorPickerSwatchState;

    type Message = ();

    fn on_focus(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state
            .interactive_state
            .set(super::InteractiveState::Focused.into());
    }

    fn on_blur(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state.interactive_state.set(InteractiveState::Normal.into());
    }

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

        if is_mouse_over && *state.started_clicking.to_ref() && mouse.left_up() {
            let Some(color_name) = context
                .attribute("color_name")
                .and_then(|value| value.as_str())
            else {
                return;
            };

            context.publish("on_select", color_name.to_owned());
        } else if is_mouse_over && !*state.started_clicking.to_ref() && mouse.left_down() {
            state
                .interactive_state
                .set(InteractiveState::MouseDown.into());
            state.started_clicking.set(true);
        } else if is_mouse_over {
            state
                .interactive_state
                .set(InteractiveState::MouseOver.into());
        } else {
            state.interactive_state.set(InteractiveState::Normal.into());
        }

        if mouse.left_up() {
            state.started_clicking.set(false);
        }
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if matches!(key.code, KeyCode::Enter)
            && let Some(color_name) = context
                .attribute("color_name")
                .and_then(|value| value.as_str())
        {
            context.publish("on_select", color_name.to_owned());
        }
    }
}

#[derive(Debug, State, Default)]
pub struct BBColorPickerSwatchState {
    interactive_state: Value<String>,
    started_clicking: Value<bool>,
}
