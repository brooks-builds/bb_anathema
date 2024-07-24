use std::fs::read_to_string;

use anathema::{backend::tui::TuiBackend, runtime::Runtime, templates::Document};
use bb_anathema::components::nav::Nav;

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

    Nav::register(&mut runtime, nav_items).unwrap();

    runtime.finish().unwrap().run();
}
