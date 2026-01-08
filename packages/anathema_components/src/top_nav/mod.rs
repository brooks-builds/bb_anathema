use anathema::{
    component::Component,
    prelude::ToSourceKind,
    runtime::{Builder, Error},
};

pub struct BBTopNav;

impl Component for BBTopNav {
    type State = ();

    type Message = ();
}

pub fn register_to(builder: &mut Builder<()>) -> Result<(), Error> {
    let template = include_str!("./template.aml");
    builder.prototype("bb_topnav", template.to_template(), || BBTopNav, || ())
}
