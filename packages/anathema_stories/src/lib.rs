use anathema::{
    prelude::{Backend, Document, TuiBackend},
    runtime::Runtime,
};
use eyre::{Context, Result};

mod app;

pub fn run() -> Result<()> {
    let doc = Document::new("@app");
    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_mouse()
        .enable_raw_mode()
        .hide_cursor()
        .finish()?;

    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);

    bb_anathema_components::register_all(&mut builder)?;
    app::register(&mut builder)?;

    builder
        .finish(&mut backend, |runtime, backend| runtime.run(backend))
        .context("running the app")
}
