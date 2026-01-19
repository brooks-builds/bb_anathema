use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct BBInputStory;

impl Component for BBInputStory {
    type State = BBInputStoryState;

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
                let initial_value = state.attribute_initial_value.to_ref();
                let aml = generate_aml(label.as_str(), initial_value.as_str());

                state.aml.set(aml);
                state.show_aml.set(true);
            }
            "remove_aml" => state.show_aml.set(false),
            "set_label" => {
                let label = event.data_checked::<String>().cloned().unwrap_or_default();

                state.attribute_label.set(label);
            }
            "set_initial_value" => {
                let value = event.data_checked::<String>().cloned().unwrap_or_default();

                state.attribute_initial_value.set(value);
            }
            _ => (),
        }
    }
}

#[derive(Debug, State)]
pub struct BBInputStoryState {
    show_preview: Value<bool>,
    aml: Value<String>,
    show_aml: Value<bool>,
    attribute_label: Value<String>,
    attribute_initial_value: Value<String>,
}

impl Default for BBInputStoryState {
    fn default() -> Self {
        let show_preview = Value::new(true);
        let aml = Value::default();
        let show_aml = Value::new(false);
        let attribute_label = Value::new("input value".to_owned());
        let attribute_initial_value = Value::new("input_value".to_owned());

        Self {
            show_preview,
            aml,
            show_aml,
            attribute_label,
            attribute_initial_value,
        }
    }
}

impl BBAppComponent for BBInputStory {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "BBInputStory",
            "templates/routes/stories/bb_input_story.aml",
            Self,
            BBInputStoryState::default(),
        )?;

        Ok(())
    }
}

fn generate_aml(label: &str, initial_value: &str) -> String {
    let mut aml = String::from("@BBBlock [");
    let mut previously_added_attribute = false;

    if !label.is_empty() {
        aml.push_str(&format!("label: \"{label}\""));
        previously_added_attribute = true;
    }

    if !initial_value.is_empty() {
        if previously_added_attribute {
            aml.push_str(", ");
        }

        aml.push_str(&format!("initial_value: \"{initial_value}\""));
    }

    aml.push(']');

    aml
}
