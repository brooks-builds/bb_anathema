use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct BBHeadingStory;

impl Component for BBHeadingStory {
    type State = BBHeadingState;

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
                let text = state.attribute_text.to_ref();
                let padding_top = state.attribute_padding_top.to_ref();
                let padding_bottom = state.attribute_padding_bottom.to_ref();
                let aml = generate_aml(
                    text.as_str(),
                    padding_top.as_int().unwrap_or_default(),
                    padding_bottom.as_int().unwrap_or_default(),
                );

                state.aml.set(aml);
                state.show_aml.set(true);
            }
            "remove_aml" => state.show_aml.set(false),
            "set_text" => {
                let text = event.data_checked::<String>().cloned().unwrap_or_default();

                state.attribute_text.set(text);
            }
            "set_padding_top" => {
                let padding = event
                    .data_checked::<String>()
                    .map(|input| input.parse::<i64>().unwrap_or_default())
                    .unwrap_or_default();

                state.attribute_padding_top.set(padding);
            }
            "set_padding_bottom" => {
                let padding = event
                    .data_checked::<String>()
                    .map(|input| input.parse::<i64>().unwrap_or_default())
                    .unwrap_or_default();

                state.attribute_padding_bottom.set(padding);
            }
            _ => (),
        }
    }
}

#[derive(Debug, State)]
pub struct BBHeadingState {
    show_preview: Value<bool>,
    aml: Value<String>,
    show_aml: Value<bool>,
    attribute_text: Value<String>,
    attribute_padding_top: Value<i64>,
    attribute_padding_bottom: Value<i64>,
}

impl Default for BBHeadingState {
    fn default() -> Self {
        let show_preview = Value::new(true);
        let aml = Value::default();
        let show_aml = Value::new(false);
        let attribute_text = Value::new("Heading".to_owned());
        let attribute_padding_top = Value::new(2);
        let attribute_padding_bottom = Value::new(2);

        Self {
            show_preview,
            aml,
            show_aml,
            attribute_text,
            attribute_padding_top,
            attribute_padding_bottom,
        }
    }
}

impl BBAppComponent for BBHeadingStory {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "BBHeadingStory",
            "templates/routes/stories/bb_heading_story.aml",
            Self,
            BBHeadingState::default(),
        )?;

        Ok(())
    }
}

fn generate_aml(text: &str, padding_top: i64, padding_bottom: i64) -> String {
    let mut aml = String::from("@BBHeading [");
    let mut previously_added_attribute = false;

    if !text.is_empty() {
        aml.push_str(&format!("text: \"{text}\""));
        previously_added_attribute = true;
    }

    if padding_top != 0 {
        if previously_added_attribute {
            aml.push_str(", ");
        } else {
            previously_added_attribute = true;
        }
        aml.push_str(&format!("padding_top: {padding_top}"));
    }

    if padding_bottom != 0 {
        if previously_added_attribute {
            aml.push_str(", ");
        }

        aml.push_str(&format!("padding_bottom: {padding_bottom}"));
    }

    aml.push(']');

    aml
}
