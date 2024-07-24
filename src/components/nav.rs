use std::error::Error;

use anathema::{
    backend::tui::TuiBackend,
    component::{Component, ComponentId},
    runtime::RuntimeBuilder,
    state::{List, State, Value},
};

pub struct Nav {
    pub observers: Vec<ComponentId<String>>,
}

impl Nav {
    pub fn new(observers: Vec<ComponentId<String>>) -> Self {
        Self { observers }
    }

    pub fn register(
        runtime: &mut RuntimeBuilder<TuiBackend>,
        routes: Vec<String>,
    ) -> Result<ComponentId<String>, impl Error> {
        runtime.register_component(
            "bb_nav",
            "templates/nav.aml",
            Self::new(vec![]),
            NavState::new(routes, "Home".to_owned()),
        )
    }
}

impl Component for Nav {
    type State = NavState;

    type Message = String;

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        _state: &mut Self::State,
        mut elements: anathema::widgets::Elements<'_, '_>,
        context: anathema::prelude::Context<'_>,
    ) {
        let query = elements.query().at_position(mouse.pos());

        query.each(|_element, attributes| {
            let Some(value) = attributes.value() else {
                return;
            };

            value.str_for_each(|v| {
                self.observers.iter().for_each(|observer| {
                    context
                        .emitter
                        .emit(observer.to_owned(), v.to_owned())
                        .unwrap()
                });
            })
        })
    }
}

#[derive(State)]
pub struct NavState {
    pub routes: Value<List<String>>,
    pub current: Value<String>,
}

impl NavState {
    pub fn new(routes: Vec<String>, current: String) -> Self {
        let routes = List::from_iter(routes.into_iter());
        let current = Value::new(current);

        Self { routes, current }
    }
}
