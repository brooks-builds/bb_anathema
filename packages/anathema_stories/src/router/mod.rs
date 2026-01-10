pub mod bb_topnav;
pub mod home;

use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct Router(Route);

impl Component for Router {
    type State = RouterState;

    type Message = String;

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        state.path.set(message);
    }
}

impl BBAppComponent for Router {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> std::result::Result<(), anathema::runtime::Error> {
        builder.component(
            "Router",
            "templates/router.aml",
            Self(Route::Home),
            RouterState::new(),
        )?;

        Ok(())
    }
}

#[derive(Debug, State)]
pub struct RouterState {
    path: Value<String>,
}

impl RouterState {
    pub fn new() -> Self {
        let path = Value::new("/".to_owned());

        Self { path }
    }
}

pub enum Route {
    Home,
}
