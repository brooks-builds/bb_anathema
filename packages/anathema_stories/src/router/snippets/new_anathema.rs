use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct NewAnathemaSnippet;

impl BBAppComponent for NewAnathemaSnippet {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "new_anathema_snippet",
            "templates/routes/snippets/new_anathema.aml",
            Self,
            NewAnathemaSnippetState::new(),
        )?;

        Ok(())
    }
}

impl Component for NewAnathemaSnippet {
    type State = NewAnathemaSnippetState;

    type Message = ();
}

#[derive(Debug, State)]
pub struct NewAnathemaSnippetState {
    snippet: Value<String>,
}

impl NewAnathemaSnippetState {
    pub fn new() -> Self {
        let snippet = r#"
use anathema::{
    component::Component,
    prelude::{Backend, Document, SourceKind, ToSourceKind, TuiBackend},
    runtime::Runtime,
};

fn main() {
    let doc = Document::new("@app");
    let mut backend = TuiBackend::builder()
        .enable_alt_screen()
        .enable_mouse()
        .enable_raw_mode()
        .hide_cursor()
        .finish()
        .unwrap();

    backend.finalize();

    let mut builder = Runtime::builder(doc, &backend);

    builder.component("app", App::template(), App, ()).unwrap();

    builder
        .finish(&mut backend, |runtime, backend| runtime.run(backend))
        .unwrap();
}

struct App;

impl App {
    pub fn template() -> SourceKind {
        r#"
text "hello world"
        "\#
        .to_template()
    }
}

impl Component for App {
    type State = ();

    type Message = ();
}
        "#;

        Self {
            snippet: Value::new(snippet.to_owned()),
        }
    }
}
