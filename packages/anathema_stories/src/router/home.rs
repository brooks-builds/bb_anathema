use anathema::component::Component;
use bb_anathema_components::BBAppComponent;

pub struct Home;

impl Component for Home {
    type State = ();

    type Message = ();

    fn accept_focus(&self) -> bool {
        false
    }
}

impl BBAppComponent for Home {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component("home", "templates/routes/home.aml", Self, ())?;
        Ok(())
    }
}
