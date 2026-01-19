use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct BBLinkStory;

impl Component for BBLinkStory {
    type State = BBLinkStoryState;

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
                let nav_to = state.attribute_nav_to.to_ref();
                let aml = generate_aml(nav_to.as_str());

                state.aml.set(aml);
                state.show_aml.set(true);
            }
            "remove_aml" => state.show_aml.set(false),
            "set_nav_to" => {
                let nav_to = event.data_checked::<String>().cloned().unwrap_or_default();

                state.attribute_nav_to.set(nav_to);
            }
            _ => (),
        }
    }
}

#[derive(Debug, State)]
pub struct BBLinkStoryState {
    show_preview: Value<bool>,
    aml: Value<String>,
    show_aml: Value<bool>,
    attribute_nav_to: Value<String>,
}

impl Default for BBLinkStoryState {
    fn default() -> Self {
        let show_preview = Value::new(true);
        let aml = Value::default();
        let show_aml = Value::new(false);
        let attribute_nav_to = Value::new("route".to_owned());

        Self {
            show_preview,
            aml,
            show_aml,
            attribute_nav_to,
        }
    }
}

impl BBAppComponent for BBLinkStory {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "BBLinkStory",
            "templates/routes/stories/bb_link_story.aml",
            Self,
            BBLinkStoryState::default(),
        )?;

        Ok(())
    }
}

fn generate_aml(nav_to: &str) -> String {
    format!("@BBlink [nav_to: \"{nav_to}\"]\n    text \"Child component\"")
}
