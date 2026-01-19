use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct NewComponent;

impl Component for NewComponent {
    type State = NewComponentState;

    type Message = ();
}

impl BBAppComponent for NewComponent {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "new_component",
            "templates/routes/snippets/new_component.aml",
            Self,
            NewComponentState::default(),
        )?;

        Ok(())
    }
}

#[derive(Debug, State)]
pub struct NewComponentState {
    rust_code: Value<String>,
    aml_code: Value<String>,
}

impl Default for NewComponentState {
    fn default() -> Self {
        Self {
            rust_code: Value::new(rust_code().to_owned()),
            aml_code: Value::new(aml_code().to_owned()),
        }
    }
}

fn rust_code() -> &'static str {
    r#"use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct BBComponentStory;

impl Component for BBComponentStory {
    type State = BBComponentState;

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "toggle_preview" => {
                let show_preview = *state.show_preview.to_ref();

                state.show_preview.set(!show_preview);
            }
            "show_aml" => {
                let max_height = *state.attribute_height.to_ref();
                let header = *state.attribute_header.to_ref();
                let scroll = *state.attribute_scroll.to_ref();
                let title = &*state.attribute_title.to_ref();
                let copy = *state.attribute_copy.to_ref();
                let value = &*state.attribute_value.to_ref();
                let aml = generate_aml(max_height, header, scroll, title, copy, value);

                state.aml.set(aml);
                state.show_aml.set(true);
            }
            "remove_aml" => state.show_aml.set(false),
            _ => (),
        }
    }
}

#[derive(Debug, State)]
pub struct BBComponentState {
    show_preview: Value<bool>,
    aml: Value<String>,
    show_aml: Value<bool>,
}

impl Default for BBComponentState {
    fn default() -> Self {
        let show_preview = Value::new(true);
        let aml = Value::default();
        let show_aml = Value::new(false);

        Self {
            show_preview,
            aml,
            show_aml,
        }
    }
}

impl BBAppComponent for BBComponentStory {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "BBComponentStory",
            "templates/routes/stories/bb_component_story.aml",
            Self,
            BBComponentState::default(),
        )?;

        Ok(())
    }
}

fn generate_aml(
    max_height: u16,
    header: bool,
    scroll: bool,
    title: &str,
    copy: bool,
    value: &str,
) -> String {
    let mut aml = String::from("@BBBlock [");
    let mut previously_added_attribute = false;

    if max_height > 0 {
        aml.push_str(&format!("max_height: {max_height}"));
        previously_added_attribute = true;
    }

    if header {
        if previously_added_attribute {
            aml.push_str(", ");
        } else {
            previously_added_attribute = true;
        }
        aml.push_str(&format!("header: {header}"));
    }

    if scroll {
        if previously_added_attribute {
            aml.push_str(", ");
        } else {
            previously_added_attribute = true;
        }
        aml.push_str(&format!("scroll: {scroll}"));
    }

    if !title.is_empty() {
        if previously_added_attribute {
            aml.push_str(", ");
        } else {
            previously_added_attribute = true;
        }
        aml.push_str(&format!("title: {title}"));
    }

    if copy {
        if previously_added_attribute {
            aml.push_str(", ");
        } else {
            previously_added_attribute = true;
        }
        aml.push_str(&format!("copy: {copy}"));
    }

    if !value.is_empty() {
        if previously_added_attribute {
            aml.push_str(", ");
        }
        aml.push_str(&format!("value: {value}"));
    }

    aml.push(']');

    aml
}
"#
}

fn aml_code() -> &'static str {
    r#"vstack
vstack
	@BBHeading [text: "Component"]
	if state.show_preview
		@BBButton (click->toggle_preview) [label: "Turn off preview"]
	else
		@BBButton (click->toggle_preview) [label: "Turn on preview"]
	if state.show_preview
		text "snippet goes here"
		
	@BBHeading [text: "Attributes"]
	@BBHeading [text: "Events"]
	if state.show_aml
		@BBBlock (copied->remove_aml) [value: state.aml, header: true, copy: true, title: "BBBlock AML"]
	else 
		@BBButton (click->show_aml) [label: "generate aml"]
"#
}
