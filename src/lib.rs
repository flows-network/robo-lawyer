use slack_flows::{listen_to_channel, send_message_to_channel};
use openai_flows::{CompletionRequest, create_completion};

#[no_mangle]
pub fn run() {
    listen_to_channel("secondstate", "robo-lawyer", |sm| {
        let cr = CompletionRequest {
            prompt: "I want you to act as my legal advisor. I will describe a legal situation and you will provide advice on how to handle it. You should only reply with your advice, and nothing else. Do not write explanations. My request is \"".to_owned() + &sm.text + "\"",
            max_tokens: 2048,
            ..Default::default()
        };
        let r = create_completion("RoboLawyer", cr);
        r.iter().for_each(|c| {
            send_message_to_channel("secondstate", "robo-lawyer", c.to_string());
        });
    });
}
