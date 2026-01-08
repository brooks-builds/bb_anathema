use anathema::runtime::{Builder, Error};

pub mod top_nav;

pub fn register_all(builder: &mut Builder<()>) -> Result<(), Error> {
    top_nav::register_to(builder)?;

    Ok(())
}
