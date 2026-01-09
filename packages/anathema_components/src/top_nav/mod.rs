use anathema::{
    component::Component,
    prelude::ToSourceKind,
    runtime::{Builder, Error},
};

use crate::BBComponent;

pub struct BBTopNav;

impl Component for BBTopNav {
    type State = ();

    type Message = ();
}

impl BBComponent for BBTopNav {
    fn register_to(builder: &mut Builder<()>) -> Result<(), Error> {
        builder.prototype(Self::ident(), Self::load_template().to_template(), || Self, || ())
    }

    fn load_template() -> &'static str {
        let template = include_str!("./template.aml");
        template
    }

    fn ident() -> &'static str {
        "bb_topnav"
    }


}
