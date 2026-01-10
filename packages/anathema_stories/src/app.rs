use anathema::{component::Component, runtime::Builder};
use bb_anathema_components::BBAppComponent;

pub struct App;

impl Component for App {
    type State = ();

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        _state: &mut Self::State,
        _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if event.name() == "nav_to" {
            let path = event.data_checked::<String>().cloned().unwrap_or_default();

            context.components.by_name("Router").send(path);
        }
    }
}

impl BBAppComponent for App {
    fn register_to(builder: &mut Builder<()>) -> std::result::Result<(), anathema::runtime::Error> {
        builder.component("app", "templates/app.aml", Self, ())?;
        Ok(())
    }
}
