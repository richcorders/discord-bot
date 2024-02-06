use poise::serenity_prelude::*;

use crate::sql::starboard_message;
use crate::Data;

async fn create_starboard_message(ctx: &Context, msg: &Message, starboard_channel_id: ChannelId) {
    let message_channel: GuildChannel = msg
        .channel(ctx)
        .await
        .expect("Failed to get channel")
        .guild()
        .expect("Failed to get guild channel");
    let starboard_channel = ctx
        .http()
        .get_channel(starboard_channel_id)
        .await
        .expect("Failed to get channel");

    let mut author = CreateEmbedAuthor::new(&msg.author.name);
    if let Some(avatar) = &msg.author.avatar_url() {
        author = author.icon_url(avatar);
    }

    let embed = CreateEmbed::default()
        .author(author)
        .footer(CreateEmbedFooter::new(format!(
            "#{}",
            &message_channel.name,
        )))
        .timestamp(msg.timestamp)
        .description(format!("{}\n\n{}", &msg.content_safe(ctx), &msg.link()));

    starboard_channel
        .id()
        .send_message(ctx, CreateMessage::new().embed(embed))
        .await
        .expect("Failed to send message");
}

pub async fn handle(ctx: &Context, event: &FullEvent, data: &Data) {
    let conn = &mut data.db_pool.get().unwrap();

    if let FullEvent::ReactionAdd { add_reaction } = event {
        let Some(guild_id) = add_reaction.guild_id else {
            // ignore DMs
            return;
        };

        let bot_options = data.bot_options.lock().clone();
        // get the starboard options for the guild
        let guild_options = bot_options
            .get(&guild_id)
            .expect("Failed to get guild options");

        // exit early if the starboard doesn't have options set
        let Some(starboard_options) = guild_options.starboard_options.get(&add_reaction.emoji)
        else {
            return;
        };

        let threshold = starboard_options.threshold;
        let starboard_channel_id = starboard_options.channel_id;

        let msg = add_reaction
            .message(ctx)
            .await
            .expect("Starboard: Failed to get message");

        let star_count = msg
            .reactions
            .iter()
            .filter(|r| r.reaction_type == add_reaction.emoji)
            .map(|r| r.count)
            .sum::<u64>();

        let previously_starboarded = starboard_message::exists(conn, msg.id.into()).unwrap();

        if star_count >= threshold.into() && !previously_starboarded {
            create_starboard_message(ctx, &msg, starboard_channel_id).await;
            starboard_message::upsert(
                conn,
                msg.id.into(),
                starboard_channel_id.into(),
                msg.author.id.into(),
                star_count.try_into().unwrap(),
                false,
            )
            .unwrap();
        }
    }
}
