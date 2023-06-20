use serenity::framework::standard::macros::command;
use serenity::framework::standard::{Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

struct Quote {
    content: String,
    date: Timestamp,
    author: User,
}

#[command]
async fn quote(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let mut quote = String::new();

    if let Some(i) = &msg.referenced_message {
        quote = i.content.clone();
    } else {
        quote = String::from(args.message());
    }

    if quote.is_empty() {
        msg.reply(&ctx.http, "No quote was provided!").await?;
    } else {
        quote.push_str(" added to the quote list!");
    }

    msg.reply(&ctx.http, quote).await?;
    Ok(())
}
