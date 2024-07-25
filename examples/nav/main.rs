use std::fs::read_to_string;

use anathema::{
    backend::tui::TuiBackend,
    component::{Component, ComponentId},
    runtime::Runtime,
    templates::Document,
};
use bb_anathema::components::{
    message::{Message, MessageMessage},
    nav::Nav,
};

fn main() {
    let index_template = read_to_string("examples/nav/index.aml").unwrap();
    let doc = Document::new(index_template);
    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_mouse()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();
    let mut runtime = Runtime::builder(doc, backend);
    let nav_items = vec!["Home".to_owned(), "About".to_owned(), "Config".to_owned()];

    let message_id = Message::register(&mut runtime).unwrap();
    let home_id = runtime
        .register_component("home", "examples/nav/home.aml", Home { message_id }, ())
        .unwrap();
    let _nav_id = Nav::register(&mut runtime, nav_items, vec![home_id]).unwrap();

    runtime.finish().unwrap().run();
}

struct Home {
    pub message_id: ComponentId<MessageMessage>,
}

impl Component for Home {
    type State = ();

    type Message = String;

    fn message(
        &mut self,
        message: Self::Message,
        _state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        context: anathema::prelude::Context<'_>,
    ) {
        context
            .emitter
            .emit(
                self.message_id,
                MessageMessage::new(
                    format!("nav item '{message}' selected"),
                    bb_anathema::components::message::MessageLevel::Info,
                ),
            )
            .unwrap();
    }
}
