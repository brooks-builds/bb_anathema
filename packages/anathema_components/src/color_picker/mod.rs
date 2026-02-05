use anathema::{component::Component, state::State};
use bb_anathema_macros::BBComponent;

#[derive(BBComponent)]
#[bb_component(state = BBColorPickerState)]
pub struct BBColorPicker;

impl Component for BBColorPicker {
    type State = BBColorPickerState;

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if event.name() == "color_selected" {
            event.stop_propagation();

            let Some(color_name) = event.data_checked::<String>().cloned() else {
                return;
            };

            context.publish("on_select", color_name);
        }
    }
}

#[derive(Debug, State, Default)]
pub struct BBColorPickerState {}
