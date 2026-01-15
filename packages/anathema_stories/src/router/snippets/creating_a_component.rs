use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct CreatingAComponent;

impl Component for CreatingAComponent {
    type State = CreatingAComponentState;

    type Message = ();
}

impl BBAppComponent for CreatingAComponent {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "creating_a_component",
            "templates/routes/snippets/creating_a_component.aml",
            Self,
            CreatingAComponentState::default(),
        )?;

        Ok(())
    }
}

#[derive(Debug, State)]
pub struct CreatingAComponentState {
    rust_code: Value<String>,
    aml_code: Value<String>,
}

impl Default for CreatingAComponentState {
    fn default() -> Self {
        Self {
            rust_code: Value::new(rust_code()),
            aml_code: Value::new(aml_code()),
        }
    }
}

fn rust_code() -> String {
    r#"use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_macros::BBComponent;

#[derive(BBComponent)]
#[bb_component(state = BBCheckboxState)]
pub struct BBCheckbox;

impl Component for BBCheckbox {
    type State = BBCheckboxState;

    type Message = ();
}

#[derive(Debug, State, Default)]
pub struct BBCheckboxState {
    checked: Value<bool>,
}
        "#
    .to_owned()
}

fn aml_code() -> String {
    "".to_owned()
}
