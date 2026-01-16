use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_macros::BBComponent;

use crate::InteractiveState;

#[derive(BBComponent)]
#[bb_component(state = BBLinkState)]
pub struct BBLink;

impl Component for BBLink {
    type State = BBLinkState;

    type Message = ();

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let position = mouse.pos();
        let is_mouse_over = children
            .elements()
            .at_position(position)
            .first(|_, _| {})
            .is_some();
        let interactive_state = InteractiveState::from(state.interactive_state.to_ref().as_str());

        if is_mouse_over && mouse.left_down() {
            state
                .interactive_state
                .set(String::from(InteractiveState::MouseDown));
        } else if is_mouse_over && mouse.left_up() {
            if interactive_state.is_mouse_down() {
                nav_to(&mut context);
            }
            state
                .interactive_state
                .set(String::from(InteractiveState::Normal));
        } else if is_mouse_over {
            state
                .interactive_state
                .set(String::from(InteractiveState::MouseOver));
        } else if matches!(interactive_state, InteractiveState::MouseOver) {
            state
                .interactive_state
                .set(String::from(InteractiveState::Normal));
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
            .set(String::from(InteractiveState::Normal))
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if matches!(key.code, anathema::component::KeyCode::Enter) {
            nav_to(&mut context);
        }
    }
}

#[derive(Debug, State)]
pub struct BBLinkState {
    interactive_state: Value<String>,
}

impl BBLinkState {}

impl Default for BBLinkState {
    fn default() -> Self {
        let interactive_state = Value::new(String::from(InteractiveState::Normal));

        Self { interactive_state }
    }
}

fn nav_to(context: &mut anathema::component::Context<'_, '_, BBLinkState>) {
    let nav_to = if let Some(nav_to) = context.attribute("nav_to") {
        nav_to.as_str().to_owned().unwrap_or_default()
    } else {
        ""
    };

    context.publish("nav_to", nav_to.to_owned());
}
