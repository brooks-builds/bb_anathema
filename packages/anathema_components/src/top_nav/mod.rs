use anathema::{
    component::Component,
    prelude::ToSourceKind,
    runtime::{Builder, Error},
};
use bb_anathema_macros::BBComponent;

#[derive(BBComponent)]
pub struct BBTopNav;

impl Component for BBTopNav {
    type State = ();

    type Message = ();
}
