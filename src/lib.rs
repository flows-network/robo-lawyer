use slack_flows::{listen_to_channel, send_message_to_channel};
use openai_flows::{CompletionRequest, create_completion};
use std::env;
use dotenv::dotenv;

#[no_mangle]
pub fn run() {
    dotenv().ok();
    let workspace: String = match env::var("workspace") {
        Err(_) => "secondstate".to_string(),
        Ok(name) => name,
    };

    let channel: String = match env::var("channel") {
        Err(_) => "robo-lawyer".to_string(),
        Ok(name) => name,
    };
    let openai_key_name: String = match env::var("openai_key_name") {
        Err(_) => "chatmichael".to_string(),
        Ok(name) => name,
    };

    listen_to_channel(&workspace, &channel, |sm| {
        let cr = CompletionRequest {
            prompt: "I want you to act as my legal advisor. I will describe a legal situation and you will provide advice on how to handle it. My question is \"".to_owned() + &sm.text + "\"",
            max_tokens: 2048,
            ..Default::default()
        };
        let r = create_completion(&openai_key_name, cr);
        r.iter().for_each(|c| {
            send_message_to_channel(&workspace, &channel, c.to_string());
        });
    });
}
