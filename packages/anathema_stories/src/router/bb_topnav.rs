use anathema::component::Component;
use bb_anathema_components::BBAppComponent;

pub struct TopNavStory;

impl Component for TopNavStory {
    type State = ();

    type Message = ();
}

impl BBAppComponent for TopNavStory {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "TopNavStory",
            "templates/routes/bb_topnav_story.aml",
            Self,
            (),
        )?;
        Ok(())
    }
}
