use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct NewSnippet;

impl Component for NewSnippet {
    type State = NewSnippetState;

    type Message = ();
}

impl BBAppComponent for NewSnippet {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "new_snippet",
            "templates/routes/snippets/new_snippet.aml",
            Self,
            NewSnippetState::default(),
        )?;

        Ok(())
    }
}

#[derive(Debug, State)]
pub struct NewSnippetState {
    code: Value<String>,
}

impl Default for NewSnippetState {
    fn default() -> Self {
        let code = r#"
use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct NewSnippet;

impl Component for NewSnippet {
    type State = NewSnippetState;

    type Message = ();
}

impl BBAppComponent for NewSnippet {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "new_snippet",
            "templates/routes/snippets/new_snippet.aml",
            Self,
            NewSnippetState::default(),
        )?;

        Ok(())
    }
}

#[derive(Debug, State)]
pub struct NewSnippetState {
    code: Value<String>,
}

impl Default for NewSnippetState {
    fn default() -> Self {
        let code = r#"
        "\#;

        Self {
            code: Value::new(code.to_owned()),
        }
    }
}
        "#;

        Self {
            code: Value::new(code.to_owned()),
        }
    }
}
