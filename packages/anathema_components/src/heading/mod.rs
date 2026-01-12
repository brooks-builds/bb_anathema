use anathema::component::Component;
use bb_anathema_macros::BBComponent;

#[derive(BBComponent)]
pub struct BBHeading;

impl Component for BBHeading {
    type State = ();

    type Message = ();
}
