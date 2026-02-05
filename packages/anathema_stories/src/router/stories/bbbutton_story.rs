use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct BBButtonStory;

impl Component for BBButtonStory {
    type State = BBButtonState;

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "toggle_preview" => {
                let show_preview = *state.show_preview.to_ref();

                state.show_preview.set(!show_preview);
            }
            "show_aml" => {
                let label = state.attribute_label.to_ref();
                let disabled = *state.attribute_disabled.to_ref();
                let aml = generate_aml(label.as_str(), disabled);

                state.aml.set(aml);
                state.show_aml.set(true);
            }
            "remove_aml" => state.show_aml.set(false),
            "set_label" => {
                let label = event.data_checked::<String>().cloned().unwrap_or_default();

                state.attribute_label.set(label);
            }
            "toggle_disabled" => {
                let disabled = *state.attribute_disabled.to_ref();

                state.attribute_disabled.set(!disabled);
            }
            "handle_click" => {
                event.stop_propagation();

                let Some(button_value) = event.data_checked::<String>() else {
                    return;
                };

                state.last_click_event.set(button_value.clone());
            }
            _ => (),
        }
    }
}

#[derive(Debug, State)]
pub struct BBButtonState {
    show_preview: Value<bool>,
    aml: Value<String>,
    show_aml: Value<bool>,
    attribute_label: Value<String>,
    attribute_disabled: Value<bool>,
    attribute_value: Value<String>,
    last_click_event: Value<String>,
}

impl Default for BBButtonState {
    fn default() -> Self {
        let show_preview = Value::new(true);
        let aml = Value::default();
        let show_aml = Value::new(false);
        let attribute_label = Value::new("Button Label".to_owned());
        let attribute_disabled = Value::default();
        let attribute_value = Value::new("value".to_owned());
        let last_click_event = Value::default();

        Self {
            show_preview,
            aml,
            show_aml,
            attribute_label,
            attribute_disabled,
            attribute_value,
            last_click_event,
        }
    }
}

impl BBAppComponent for BBButtonStory {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "BBButtonStory",
            "templates/routes/stories/bb_button_story.aml",
            Self,
            BBButtonState::default(),
        )?;

        Ok(())
    }
}

fn generate_aml(label: &str, disabled: bool) -> String {
    format!(
        r#"@BBButton (click->click) [label: \"{label}\", disabled: {disabled}, value: "value"]"#
    )
}
