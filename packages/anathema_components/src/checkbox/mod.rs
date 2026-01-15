use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_macros::BBComponent;

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
        children.elements().at_position(mouse.pos()).first(|_, _| {
            let checked = *state.checked.to_ref();
            let mut started_clicking = state.started_clicking.to_mut();

            if mouse.left_down() && !*started_clicking {
                *started_clicking = true;
            } else if mouse.left_up() && *started_clicking {
                let checked = !checked;
                state.checked.set(checked);

                context.publish("on_change", checked);
            }
        });

        if mouse.left_up() {
            state.started_clicking.set(false);
        }
    }

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let checked = context
            .attribute("checked")
            .unwrap_or(&anathema::resolver::ValueKind::Bool(false));
        let checked = checked.as_bool().unwrap_or_default();

        state.checked.set(checked);
    }
}

#[derive(Debug, State, Default)]
pub struct BBCheckboxState {
    checked: Value<bool>,
    started_clicking: Value<bool>,
}
