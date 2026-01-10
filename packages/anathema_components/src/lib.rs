pub mod heading2;
pub mod link;
pub mod top_nav;

use crate::{heading2::BBH2, top_nav::BBTopNav};
use anathema::runtime::{Builder, Error};

pub fn register_all(builder: &mut Builder<()>) -> Result<(), Error> {
    BBTopNav::register_to(builder)?;
    BBH2::register_to(builder)?;
    link::BBLink::register_to(builder)?;

    Ok(())
}

pub trait BBComponent {
    fn register_to(builder: &mut Builder<()>) -> Result<(), Error>;
    fn load_template() -> &'static str;
    fn ident() -> &'static str;
}

pub trait BBAppComponent {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), Error>;
}

