pub mod heading;
pub mod input;
pub mod link;
pub mod top_nav;

use crate::{heading::BBHeading, input::BBInput, top_nav::BBTopNav};
use anathema::runtime::{Builder, Error};

pub fn register_all(builder: &mut Builder<()>) -> Result<(), Error> {
    BBTopNav::register_to(builder)?;
    BBHeading::register_to(builder)?;
    link::BBLink::register_to(builder)?;
    BBInput::register_to(builder)?;

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
