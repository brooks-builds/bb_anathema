use anathema::{component::Component, state::State};
use bb_anathema_macros::BBComponent;

#[derive(BBComponent)]
#[bb_component(state = BBColorPickerState)]
pub struct BBColorPicker;

impl Component for BBColorPicker {
    type State = BBColorPickerState;

    type Message = ();
}

#[derive(Debug, State, Default)]
pub struct BBColorPickerState {}
