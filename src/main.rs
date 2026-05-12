use borsh::{BorshDeserialize, BorshSerialize};
use lapin::{
    options::{BasicPublishOptions, QueueDeclareOptions},
    types::FieldTable,
    BasicProperties, Connection, ConnectionProperties,
};

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct UserCreatedEventMessage {
    pub user_id: String,
    pub user_name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let connection =
        Connection::connect("amqp://guest:guest@127.0.0.1:5672", ConnectionProperties::default())
            .await?;
    let channel = connection.create_channel().await?;

    channel
        .queue_declare(
            "user_created".into(),
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    for message in [
        UserCreatedEventMessage {
            user_id: "1".to_owned(),
            user_name: "2406360256-Amir".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "2".to_owned(),
            user_name: "2406360256-Budi".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "3".to_owned(),
            user_name: "2406360256-Cica".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "4".to_owned(),
            user_name: "2406360256-Dira".to_owned(),
        },
        UserCreatedEventMessage {
            user_id: "5".to_owned(),
            user_name: "2406360256-Emir".to_owned(),
        },
    ] {
        let payload = message.try_to_vec()?;

        channel
            .basic_publish(
                "".into(),
                "user_created".into(),
                BasicPublishOptions::default(),
                &payload,
                BasicProperties::default(),
            )
            .await?
            .await?;
    }

    Ok(())
}
