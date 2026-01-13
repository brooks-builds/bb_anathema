use anathema::{
    component::Component,
    runtime::Builder,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct App;

impl Component for App {
    type State = AppState;

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

    fn accept_focus(&self) -> bool {
        false
    }

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let viewport_size = context.viewport.size();
        let navbar_height = 2;
        let usable_height = viewport_size.height - navbar_height;
        let left_nav_width = 25;
        let main_width = viewport_size.width - left_nav_width - 1;

        state.height.set(usable_height);
        state.left_nav_width.set(left_nav_width);
        state.main_width.set(main_width);
    }
}

impl BBAppComponent for App {
    fn register_to(builder: &mut Builder<()>) -> std::result::Result<(), anathema::runtime::Error> {
        builder.component("app", "templates/app.aml", Self, AppState::default())?;
        Ok(())
    }
}

#[derive(Debug, State, Default)]
pub struct AppState {
    left_nav_width: Value<u16>,
    main_width: Value<u16>,
    height: Value<u16>,
}
