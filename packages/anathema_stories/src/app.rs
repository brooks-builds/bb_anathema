use anathema::{
    component::{Component, ComponentId},
    runtime::{Builder, Error},
};

pub struct App;

impl Component for App {
    type State = ();

    type Message = ();
}

pub fn register(builder: &mut Builder<()>) -> Result<ComponentId<()>, Error> {
    builder.component("app", "templates/app.aml", App, ())
}
