use anathema::{component::Component, prelude::ToSourceKind};
use bb_anathema_macros::BBComponent;

#[derive(BBComponent)]
pub struct BBTopNav;

impl Component for BBTopNav {
    type State = ();

    type Message = ();
}
