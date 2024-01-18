use poise::serenity_prelude::*;

use crate::dn::handle_dn;
use crate::{Data, Error};

pub async fn event_handler(
    _ctx: &Context,
    event: &FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    handle_dn(event, data);
    Ok(())
}
