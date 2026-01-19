pub mod block;
pub mod button;
pub mod checkbox;
pub mod heading;
pub mod input;
pub mod link;
pub mod top_nav;

use crate::{
    block::BBBlock, button::BBButton, checkbox::BBCheckbox, heading::BBHeading, input::BBInput,
    top_nav::BBTopNav,
};
use anathema::{
    resolver::ValueKind,
    runtime::{Builder, Error},
};

pub fn register_all(builder: &mut Builder<()>) -> Result<(), Error> {
    BBTopNav::register_to(builder)?;
    BBHeading::register_to(builder)?;
    link::BBLink::register_to(builder)?;
    BBInput::register_to(builder)?;
    BBBlock::register_to(builder)?;
    BBButton::register_to(builder)?;
    BBCheckbox::register_to(builder)?;

    builder.register_function("bb_insert_into", insert_into)?;

    Ok(())
}

pub trait BBComponent {
    fn register_to(builder: &mut Builder<()>) -> Result<(), Error>;
    fn load_template() -> &'static str;
    fn ident() -> &'static str;
}

pub trait BBAppComponent {
    fn register_to(builder: &mut anathema::runtime::Builder<()>) -> Result<(), Error>;
}

fn insert_into<'a>(args: &[ValueKind<'a>]) -> ValueKind<'a> {
    if args.len() != 3 {
        return ValueKind::Null;
    }

    let Some(value) = args[0].as_str() else {
        return ValueKind::Null;
    };
    let Some(index) = args[1].to_int() else {
        return ValueKind::Null;
    };
    let Ok(index) = usize::try_from(index) else {
        return ValueKind::Null;
    };
    let Some(inserting_value) = args[2].as_str() else {
        return ValueKind::Null;
    };

    let (before, after) = value.split_at(index);

    let full_value = [before, inserting_value, after].join("");

    ValueKind::Str(full_value.into())
}

#[derive(Debug, Default)]
enum InteractiveState {
    #[default]
    Normal,
    Focused,
    MouseOver,
    MouseDown,
}

impl InteractiveState {
    pub fn is_mouse_down(&self) -> bool {
        matches!(self, Self::MouseDown)
    }
}

impl From<String> for InteractiveState {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<InteractiveState> for String {
    fn from(value: InteractiveState) -> Self {
        match value {
            InteractiveState::Normal => "normal",
            InteractiveState::Focused => "focused",
            InteractiveState::MouseOver => "mouse_over",
            InteractiveState::MouseDown => "mouse_down",
        }
        .to_owned()
    }
}

impl From<&str> for InteractiveState {
    fn from(value: &str) -> Self {
        match value {
            "normal" => Self::Normal,
            "focused" => Self::Focused,
            "mouse_over" => Self::MouseOver,
            "mouse_down" => Self::MouseDown,
            _ => Self::Normal,
        }
    }
}
