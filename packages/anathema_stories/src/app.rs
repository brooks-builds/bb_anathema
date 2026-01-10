use anathema::{
    component::{Component, ComponentId},
    runtime::{Builder, Error},
};
use bb_anathema_components::BBAppComponent;

pub struct App;

impl Component for App {
    type State = ();

    type Message = ();
}

impl BBAppComponent for App {
    fn register_to(builder: &mut Builder<()>) -> Result<ComponentId<()>, Error> {
        builder.component("app", "templates/app.aml", Self, ())
    }
}
