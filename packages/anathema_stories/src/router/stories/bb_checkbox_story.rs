use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct BBCheckboxStory;

impl Component for BBCheckboxStory {
    type State = BBCheckboxState;

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
                let checked = state.attribute_checked.to_ref();
                let label = state.attribute_label.to_ref();
                let aml = generate_aml(checked.as_bool().unwrap_or_default(), label.as_str());

                state.aml.set(aml);
                state.show_aml.set(true);
            }
            "remove_aml" => state.show_aml.set(false),
            "set_label" => {
                let label = event.data_checked::<String>().cloned().unwrap_or_default();

                state.attribute_label.set(label);
            }
            "set_checked" => {
                let checked = event.data_checked::<bool>().copied().unwrap_or_default();

                state.attribute_checked.set(checked);
            }
            _ => (),
        }
    }
}

#[derive(Debug, State)]
pub struct BBCheckboxState {
    show_preview: Value<bool>,
    aml: Value<String>,
    show_aml: Value<bool>,
    attribute_checked: Value<bool>,
    attribute_label: Value<String>,
}

impl Default for BBCheckboxState {
    fn default() -> Self {
        let show_preview = Value::new(true);
        let aml = Value::default();
        let show_aml = Value::new(false);
        let attribute_checked = Value::new(true);
        let attribute_label = Value::new("checkbox".to_owned());

        Self {
            show_preview,
            aml,
            show_aml,
            attribute_checked,
            attribute_label,
        }
    }
}

impl BBAppComponent for BBCheckboxStory {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "BBCheckboxStory",
            "templates/routes/stories/bb_checkbox_story.aml",
            Self,
            BBCheckboxState::default(),
        )?;

        Ok(())
    }
}

fn generate_aml(checked: bool, label: &str) -> String {
    let mut aml = String::from("@BBCheckbox [");
    let mut previously_added_attribute = false;

    if checked {
        aml.push_str("checked: true");
        previously_added_attribute = true;
    }

    if !label.is_empty() {
        if previously_added_attribute {
            aml.push_str(", ");
        }
        aml.push_str(&format!("label: \"{label}\""));
    }

    aml.push(']');

    aml
}
