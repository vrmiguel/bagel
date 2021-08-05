mod client;
mod error;
mod method;
mod partition;

use std::env;

use client::SlackClient;
use error::Result;

use crate::partition::random_partition;

// The id of this bot
const BOT_ID: &str = "U02A31NTD6F";
const MESSAGE: &str = "👋 Bom dia! Tá na hora do encontro do <#channel>! Combinem um horário para todos tomarem um café e colocarem o papo em dia. 😊";
const CHANNEL_IDS: &str = include_str!("../channel-ids");

#[tokio::main]
async fn main() {
    let channels: Vec<_> = CHANNEL_IDS.split('\n').collect();

    for channel in channels {
        let _ = dbg!(set_up_meetings(channel).await);
    }
}

async fn set_up_meetings(channel_id: &str) -> Result<()> {
    // Get your token at `https://api.slack.com/apps/<your-bot-id>/oauth?`
    let oauth_token = env::var("SLACK_OAUTH_TOKEN").expect("SLACK_OAUTH_TOKEN not found");
    let client = SlackClient::from_key(&oauth_token);

    let mut users = client.members_of_channel(channel_id).await?;

    dbg!(&users);
    users.retain(|user_id| user_id != BOT_ID);

    let user_partitions = random_partition(&mut users, 2);
    dbg!(&user_partitions);

    for partition in user_partitions {
        eprintln!("Criando DM com {:?}", partition);

        let message = MESSAGE.replace("channel", channel_id);

        let channel_id = client.start_direct_message(partition).await?;
        dbg!(&channel_id);
        if client.post_message(&channel_id, &message).await.is_err() {
            // mande erro
        }
    }

    Ok(())
}
