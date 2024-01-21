use log::info;
use poise::serenity_prelude::*;
mod dn;
mod starboard;

use crate::Data;

pub async fn event_handler(ctx: &Context, event: &FullEvent, data: &Data) {
    if let FullEvent::Ready { data_about_bot } = event {
        info!("Logged in as {}", data_about_bot.user.name);
    }
    dn::handle(ctx, event, data).await;
    starboard::handle(ctx, event, data).await;
}
