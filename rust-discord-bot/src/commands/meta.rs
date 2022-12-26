use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    println!("Ponging out of my gourd!");
    msg.reply(&ctx.http, "Pong!").await?;

    Ok(())
}