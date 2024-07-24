use std::fs::read_to_string;

use anathema::{backend::tui::TuiBackend, runtime::Runtime, templates::Document};
use bb_anathema::components::top_nav_bar::TopNavBar;

fn main() {
    let index_template = read_to_string("examples/top_nav_bar/index.aml").unwrap();
    let doc = Document::new(index_template);
    let backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_mouse()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();
    let mut runtime = Runtime::builder(doc, backend);

    TopNavBar::register(&mut runtime).unwrap();

    runtime.finish().unwrap().run();
}
