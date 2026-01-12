use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_macros::BBComponent;

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

        if mouse.left_down() {
            children
                .elements()
                .at_position(position)
                .first(|_el, _attrs| {
                    state.mouse_down.set(true);
                });
        } else if mouse.left_up() {
            let state_mouse_down = *state.mouse_down.to_ref();

            children
                .elements()
                .at_position(position)
                .first(|_el, _attrs| {
                    if state_mouse_down {
                        let nav_to = if let Some(nav_to) = context.attribute("nav_to") {
                            nav_to.as_str().to_owned().unwrap_or_default()
                        } else {
                            ""
                        };

                        context.publish("nav_to", nav_to.to_owned());
                    }
                });

            state.mouse_down.set(false);
        }
    }

    fn accept_focus(&self) -> bool {
        false
    }
}

#[derive(Debug, State, Default)]
pub struct BBLinkState {
    mouse_down: Value<bool>,
}

impl BBLinkState {
    pub fn new() -> Self {
        Self::default()
    }
}
