use crate::top_nav::BBTopNav;
use anathema::runtime::{Builder, Error};

pub mod top_nav;

pub fn register_all(builder: &mut Builder<()>) -> Result<(), Error> {
    BBTopNav::register_to(builder)?;

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
    ) -> Result<anathema::component::ComponentId<()>, Error>;
}
