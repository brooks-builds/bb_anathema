use std::error::Error;

use anathema::{
    backend::tui::TuiBackend,
    component::{Component, ComponentId},
    runtime::RuntimeBuilder,
    state::{State, Value},
};

pub struct Message;

impl Message {
    pub fn new() -> Self {
        Self
    }

    pub fn register(
        runtime: &mut RuntimeBuilder<TuiBackend>,
    ) -> Result<ComponentId<MessageMessage>, impl Error> {
        runtime.register_component(
            "bb_message",
            "templates/message.aml",
            Self,
            MessageState::new(),
        )
    }
}

impl Component for Message {
    type State = MessageState;

    type Message = MessageMessage;

    fn message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        _elements: anathema::widgets::Elements<'_, '_>,
        _context: anathema::prelude::Context<'_>,
    ) {
        state.message.set(message.message);
        state.level.set(message.level.to_string());
        state.color.set(message.level.color());
    }
}

#[derive(State)]
pub struct MessageState {
    pub message: Value<String>,
    pub level: Value<String>,
    pub color: Value<String>,
}

impl MessageState {
    pub fn new() -> Self {
        let message = Value::new(String::new());
        let level = Value::new(MessageLevel::Normal.to_string());
        let color = Value::new(MessageLevel::Normal.color());

        Self {
            message,
            level,
            color,
        }
    }
}

pub enum MessageLevel {
    Error,
    Info,
    Normal,
}

impl MessageLevel {
    pub fn color(&self) -> String {
        match self {
            Self::Error => "red",
            Self::Info => "blue",
            Self::Normal => "white",
        }
        .to_owned()
    }
}

impl ToString for MessageLevel {
    fn to_string(&self) -> String {
        match self {
            MessageLevel::Error => "Error",
            MessageLevel::Info => "Info",
            MessageLevel::Normal => "Normal",
        }
        .to_owned()
    }
}

pub struct MessageMessage {
    pub message: String,
    pub level: MessageLevel,
}

impl MessageMessage {
    pub fn new(message: impl Into<String>, level: MessageLevel) -> Self {
        Self {
            message: message.into(),
            level,
        }
    }
}
