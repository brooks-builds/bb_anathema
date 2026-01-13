use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_macros::BBComponent;

#[derive(BBComponent)]
#[bb_component(state = BBButtonState)]
pub struct BBButton;

impl Component for BBButton {
    type State = BBButtonState;

    type Message = ();

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let position = mouse.pos();

        children.elements().at_position(position).first(|_, _| {
            if mouse.left_down() {
                state.start_click.set(true);
                state.foreground.set("#00ff00".to_owned());
            } else if mouse.left_up() && *state.start_click.to_ref() {
                context.publish("click", ());
            }
        });

        if mouse.left_up() {
            state.start_click.set(false);
            state.foreground.set("black".to_owned());
        }
    }
}

#[derive(Debug, State)]
pub struct BBButtonState {
    foreground: Value<String>,
    start_click: Value<bool>,
}

impl BBButtonState {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for BBButtonState {
    fn default() -> Self {
        let foreground = Value::new("black".to_owned());

        Self {
            foreground,
            start_click: Default::default(),
        }
    }
}
