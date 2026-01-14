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

pub struct BBBlockStory;

impl Component for BBBlockStory {
    type State = BBBlockStoryState;

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
            "set_max_height" => {
                let Some(height) = event.data_checked::<String>() else {
                    return;
                };
                let height = height.parse::<u16>().unwrap_or(0);

                state.attribute_height.set(height);
            }
            "set_header" => {
                let header = event.data_checked::<String>().cloned().unwrap_or_default();
                let header = header == "true";

                state.attribute_header.set(header);
            }
            "set_scroll" => {
                let scroll = event.data_checked::<String>().cloned().unwrap_or_default() == "true";

                state.attribute_scroll.set(scroll);
            }
            "set_title" => {
                let title = event.data_checked::<String>().cloned().unwrap_or_default();

                state.attribute_title.set(title);
            }
            "set_copy" => {
                let copy = event.data_checked::<String>().cloned().unwrap_or_default() == "true";

                state.attribute_copy.set(copy);
            }
            "set_value" => {
                let value = event.data_checked::<String>().cloned().unwrap_or_default();

                state.attribute_value.set(value);
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
pub struct BBBlockStoryState {
    show_preview: Value<bool>,
    attribute_height: Value<u16>,
    attribute_header: Value<bool>,
    attribute_scroll: Value<bool>,
    attribute_title: Value<String>,
    attribute_copy: Value<bool>,
    attribute_value: Value<String>,
    aml: Value<String>,
    show_aml: Value<bool>,
}

impl Default for BBBlockStoryState {
    fn default() -> Self {
        let show_preview = Value::new(true);
        let attribute_height = Value::new(10);
        let attribute_header = Value::new(true);
        let attribute_scroll = Value::new(true);
        let attribute_title = Value::new("Block Title".to_owned());
        let attribute_copy = Value::new(true);
        let attribute_value = Value::new("I am the contents of the block".to_owned());
        let aml = Value::default();
        let show_aml = Value::new(false);

        Self {
            show_preview,
            attribute_height,
            attribute_header,
            attribute_scroll,
            attribute_title,
            attribute_copy,
            attribute_value,
            aml,
            show_aml,
        }
    }
}

impl BBAppComponent for BBBlockStory {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "BBBlockStory",
            "templates/routes/stories/block_story.aml",
            Self,
            BBBlockStoryState::default(),
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
	@BBHeading [text: "BBBlock"]
	if state.show_preview
		@BBButton (click->toggle_preview) [label: "Turn off preview"]
	else
		@BBButton (click->toggle_preview) [label: "Turn on preview"]
	if state.show_preview
		@BBBlock [max_height: state.attribute_height, header: state.attribute_header, scroll: state.attribute_scroll, title: state.attribute_title, copy: state.attribute_copy, value: state.attribute_value]
		
	@BBHeading [text: "Attributes"]
	@BBInput (on_change->set_max_height) [label: "max_height", initial_value: to_str(state.attribute_height)]
	@BBInput (on_change->set_header) [label: "header", initial_value: to_str(state.attribute_header)]
	@BBInput (on_change->set_scroll) [label: "scroll", initial_value: to_str(state.attribute_scroll)]
	@BBInput (on_change->set_title) [label: "title", initial_value: state.attribute_title]
	@BBInput (on_change->set_copy) [label: "copy", initial_value: to_str(state.attribute_copy)]
	@BBInput (on_change->set_value) [label: "value", initial_value: state.attribute_value]
	@BBHeading [text: "Events"]
	text "copied -> ()"
	if state.show_aml
		@BBBlock (copied->remove_aml) [value: state.aml, header: true, copy: true, title: "BBBlock AML"]
	else 
		@BBButton (click->show_aml) [label: "generate aml"]

"#
}
