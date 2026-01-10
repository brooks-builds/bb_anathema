use anathema::component::Component;
use bb_anathema_macros::BBComponent;

#[derive(BBComponent)]
pub struct BBH2;

impl Component for BBH2 {
    type State = ();

    type Message = ();
}
