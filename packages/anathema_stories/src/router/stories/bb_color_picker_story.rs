use anathema::{
    component::Component,
    state::{List, State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct BBColorPickerStory;

impl Component for BBColorPickerStory {
    type State = BBColorPickerStoryState;

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
                let aml = generate_aml();

                state.aml.set(aml);
                state.show_aml.set(true);
            }
            "remove_aml" => state.show_aml.set(false),
            _ => (),
        }
    }
}

#[derive(Debug, State)]
pub struct BBColorPickerStoryState {
    show_preview: Value<bool>,
    aml: Value<String>,
    show_aml: Value<bool>,
    attribute_colors: Value<List<String>>,
    attribute_selected: Value<String>,
}

impl Default for BBColorPickerStoryState {
    fn default() -> Self {
        let show_preview = Value::new(true);
        let aml = Value::default();
        let show_aml = Value::new(false);
        let mut attribute_colors = Value::new(List::empty());
        let attribute_selected = Value::new("red".to_owned());

        attribute_colors.push("red".to_owned());
        attribute_colors.push("green".to_owned());
        attribute_colors.push("yellow".to_owned());
        attribute_colors.push("blue".to_owned());
        attribute_colors.push("magenta".to_owned());
        attribute_colors.push("cyan".to_owned());

        Self {
            show_preview,
            aml,
            show_aml,
            attribute_colors,
            attribute_selected,
        }
    }
}

impl BBAppComponent for BBColorPickerStory {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "BBColorPickerStory",
            "templates/routes/stories/bb_color_picker.aml",
            Self,
            BBColorPickerStoryState::default(),
        )?;

        Ok(())
    }
}

fn generate_aml() -> String {
    "@BBColorPicker".to_owned()
}
