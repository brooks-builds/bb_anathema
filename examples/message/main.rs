use std::fs::read_to_string;

use anathema::{backend::tui::TuiBackend, runtime::Runtime, templates::Document};
use bb_anathema::components::message::{Message, MessageLevel, MessageMessage};

fn main() {
    let index_template = read_to_string("examples/message/index.aml").unwrap();
    let doc = Document::new(index_template);
    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_mouse()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();
    let mut runtime = Runtime::builder(doc, backend);

    let message_id = Message::register(&mut runtime).unwrap();

    runtime
        .emitter()
        .emit(
            message_id,
            MessageMessage::new("This is an error message", MessageLevel::Error),
        )
        .unwrap();

    runtime.finish().unwrap().run();
}
