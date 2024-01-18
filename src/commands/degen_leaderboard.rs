use std::fmt::Write;

use diesel::prelude::*;
use diesel::{QueryDsl, SelectableHelper};
use poise::CreateReply;

use crate::sql::models::DegenLeaderboard;
use crate::{Context, Error};

/// The Richcord degen leaderboard
#[poise::command(slash_command, subcommands("register", "show"))]
pub async fn degen_leaderboard(_: Context<'_>) -> Result<(), Error> {
    Ok(())
}

/// Show the Richcord degen leaderboard
#[poise::command(slash_command)]
async fn show(ctx: Context<'_>) -> Result<(), Error> {
    use crate::sql::schema::degen_leaderboard::dsl::*;
    let conn = &mut ctx.data().db_pool.clone().get().unwrap();
    let leaderboard = degen_leaderboard
        .select(DegenLeaderboard::as_select())
        .order_by(score)
        .load(conn);

    let leaderboard = leaderboard.map(|leaderboard| {
        leaderboard
            .chunks(10)
            .enumerate()
            .map(|(chunk_i, chunk)| {
                chunk.iter().fold(
                    format!("**Richcord degen leaderboard**: Page {}\n\n", chunk_i + 1),
                    |mut output, data| {
                        let _ = writeln!(output, "{:.1} - <@{}>", data.score, data.id);
                        output
                    },
                ) + "\n**lower score = more degen**\n[take it yourself](https://senguyen1011.github.io/state-purity/) & register your own score with \n`/degen_leaderboard register <weighted_score>`"
            })
            .collect::<Vec<String>>()
    });

    if let Ok(leaderboard) = leaderboard {
        let pages: Vec<&str> = leaderboard.iter().map(AsRef::as_ref).collect();

        if pages.is_empty() {
            ctx.send(
                CreateReply::default()
                    .ephemeral(true)
                    .content("No leaderboard data."),
            )
            .await?;
            return Ok(());
        }

        poise::builtins::paginate(ctx, &pages).await?;
    } else {
        ctx.send(
            CreateReply::default()
                .ephemeral(true)
                .content("Failed to fetch leaderboard data."),
        )
        .await?;
    }
    Ok(())
}

/// Register your score to the Richcord degen leaderboard
#[poise::command(slash_command)]
async fn register(
    ctx: Context<'_>,
    #[rename = "weighted_score"] score_value: f64,
) -> Result<(), Error> {
    if !(0.0..=100.0).contains(&score_value) {
        ctx.send(
            CreateReply::default()
                .ephemeral(true)
                .content("Score must be between 0 and 100."),
        )
        .await?;
        return Ok(());
    }

    use crate::sql::schema::degen_leaderboard::dsl::*;
    let conn = &mut ctx.data().db_pool.clone().get().unwrap();

    let user = ctx.author();

    let _ = diesel::insert_into(degen_leaderboard)
        .values((
            id.eq::<i64>(user.id.into()),
            score.eq(score_value),
            time_stamp.eq(diesel::dsl::now),
        ))
        .on_conflict(id)
        .do_update()
        .set(score.eq(score_value))
        .execute(conn);

    ctx.send(
        CreateReply::default()
            .ephemeral(true)
            .content(format!("You registered a score of {:.1}.", score_value)),
    )
    .await?;

    Ok(())
}
