use anathema::{
    component::{Component, KeyCode},
    default_widgets::Overflow,
    state::{State, Value},
};
use arboard::Clipboard;
use bb_anathema_macros::BBComponent;

#[derive(BBComponent)]
#[bb_component(state = BBBlockState)]
pub struct BBBlock;

impl Component for BBBlock {
    type State = BBBlockState;

    type Message = ();

    fn accept_focus(&self) -> bool {
        true
    }

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        _state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        children
            .elements()
            .at_position(mouse.pos())
            .by_tag("overflow")
            .first(|_el, _el_attr| {
                if mouse.left_down() {
                    context.components.by_name("BBBlock").focus();
                }
            });
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        _state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let code = key.code;

        children
            .elements()
            .by_tag("overflow")
            .first(|element, _overflow_attributes| {
                let Some(overflow) = element.try_to::<Overflow>() else {
                    return;
                };

                match code {
                    KeyCode::Down => {
                        overflow.scroll_down();
                    }
                    KeyCode::Up => overflow.scroll_up(),
                    KeyCode::Char('k') => overflow.scroll_up(),
                    KeyCode::Char('j') => overflow.scroll_down(),
                    _ => (),
                }
            });
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if event.name() != "copy" {
            return;
        }
        let Some(value) = context.attribute("value") else {
            return;
        };
        let Some(value) = value.as_str() else { return };
        if let Ok(mut clipboard) = Clipboard::new() {
            clipboard.set_text(value).ok();
        }
    }
}

#[derive(Debug, State, Default)]
pub struct BBBlockState {
    started_clicking: Value<bool>,
    foreground: Value<String>,
    width: Value<u16>,
    height: Value<u16>,
}

impl BBBlockState {
    pub fn new() -> Self {
        let started_clicking = Value::default();
        let foreground = Value::new(String::from("white"));
        let width = Value::default();
        let height = Value::default();

        Self {
            started_clicking,
            foreground,
            width,
            height,
        }
    }
}
