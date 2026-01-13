use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct TopNavStory;

impl Component for TopNavStory {
    type State = TopNavStoryState;

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if event.name() == "app_name_changed" {
            let Some(app_name) = event.data_checked::<String>() else {
                return;
            };

            state.app_name.set(app_name.to_owned());
            state.code.set(create_example(app_name));
        } else if event.name() == "generate_aml" {
            let code = create_example(state.app_name.to_ref().as_str());

            state.code.set(code);
            state.show_code.set(true);
        } else if event.name() == "code_copied" {
            state.show_code.set(false);
        }
    }
}

impl BBAppComponent for TopNavStory {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "TopNavStory",
            "templates/routes/bb_topnav_story.aml",
            Self,
            TopNavStoryState::new(),
        )?;
        Ok(())
    }
}

#[derive(State, Debug)]
pub struct TopNavStoryState {
    app_name: Value<String>,
    code: Value<String>,
    show_code: Value<bool>,
}

impl TopNavStoryState {
    pub fn new() -> Self {
        let app_name = String::from("App Name");
        let code = Value::new(create_example(&app_name));
        let show_code = Value::new(false);

        Self {
            app_name: Value::new(app_name),
            code,
            show_code,
        }
    }
}

fn create_example(app_name: &str) -> String {
    format!("@BBTopNav [app_name: \"{app_name}\"]")
}
