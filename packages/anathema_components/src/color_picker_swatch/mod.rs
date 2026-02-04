use anathema::{
    component::Component,
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
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state
            .interactive_state
            .set(super::InteractiveState::Focused.into());
    }

    fn on_blur(
        &mut self,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state.interactive_state.set(InteractiveState::Normal.into());
    }
}

#[derive(Debug, State, Default)]
pub struct BBColorPickerSwatchState {
    interactive_state: Value<String>,
}
