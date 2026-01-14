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
    rust_code: Value<String>,
    aml_code: Value<String>,
}

impl Default for NewSnippetState {
    fn default() -> Self {
        let rust_code = r#"
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
    rust_code: Value<String>,
    aml_code: Value<String>,
}

impl Default for NewSnippetState {
    fn default() -> Self {
        let rust_code = r#""\#;
        let aml_code = r#""\#;

        Self {
            rust_code: Value::new(rust_code.to_owned()),
            aml_code: Value::new(aml_code.to_owned()),
        }
    }
}
        "#;
        let aml_code = r#"hstack
    border [sides: "right", width: attributes.width / 2]
        expand [axis: "vert"]
            vstack
                @BBHeading [text: "Instruction", padding_bottom: 1]
    vstack 
        @BBHeading [text: "Code", padding_bottom: 1]
        "#;

        Self {
            rust_code: Value::new(rust_code.to_owned()),
            aml_code: Value::new(aml_code.to_owned()),
        }
    }
}
