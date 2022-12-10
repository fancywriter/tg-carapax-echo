use std::env;

use carapax::{
    webhook::run_server,
    methods::SendMessage,
    types::{ChatId, Text},
    Api, App, Context, ExecuteError, Ref,
};

async fn echo(api: Ref<Api>, chat_id: ChatId, message: Text) -> Result<(), ExecuteError> {
    let method = SendMessage::new(chat_id, message.data);
    api.execute(method).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let token = env::var("TOKEN").expect("TOKEN is not set");
    let port = env::var("PORT").expect("POST is not set").parse::<u16>().expect("Post must be an integer");
    let api = Api::new(token).expect("Failed to create API");

    let mut context = Context::default();
    context.insert(api.clone());

    let app = App::new(context, echo);
    run_server(([127, 0, 0, 1], port), "/echo", app ).await.unwrap();
}
