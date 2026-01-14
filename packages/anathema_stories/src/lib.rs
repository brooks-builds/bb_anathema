mod app;
mod router;

use crate::{
    app::App,
    router::{
        Router,
        bb_topnav::TopNavStory,
        home::Home,
        snippets::{
            new_anathema::NewAnathemaSnippet, new_component::NewComponent, new_snippet::NewSnippet,
        },
        stories::BBBlock::BBBlockStory,
    },
};
use anathema::{
    prelude::{Backend, Document, TuiBackend},
    runtime::Runtime,
};
use bb_anathema_components::BBAppComponent;
use eyre::{Context, Result};

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

    App::register_to(&mut builder)?;
    Router::register_to(&mut builder)?;
    Home::register_to(&mut builder)?;
    TopNavStory::register_to(&mut builder)?;
    NewAnathemaSnippet::register_to(&mut builder)?;
    NewSnippet::register_to(&mut builder)?;
    NewComponent::register_to(&mut builder)?;
    BBBlockStory::register_to(&mut builder)?;

    builder
        .finish(&mut backend, |runtime, backend| runtime.run(backend))
        .context("running the app")
}
