# Robo Lawyer for your slack channel, powered by ChatFPT

[Deploy this function on flows.network](#deploy-chatgpt-slack-bot-on-your-slack-channel), and you will get a Slack bot that uses ChatGPT to respond to **every legal question** in your Slack workspace automatically.


See the following example

<img width="711" alt="image" src="https://user-images.githubusercontent.com/45785633/224661731-f0164571-57f4-4ae5-a7a2-87264ea3692a.png">



> Please note, the answer from CHatGPT is only for reference.

## Prerequisite 

You will need an [OpenAI API key](https://openai.com/blog/openai-api). If you do not already have one, [sign up here](https://platform.openai.com/signup).

## Deploy ChatGPT Slack bot on your Slack Channel

To install this Robot Lawyer ChatGPT Slack App, we will use [flows.network](https://flows.network/), a serverless platform that makes deploying your own app quick and easy in just three steps.

### Fork this repo

Fork [this repo](https://github.com/flows-network/robo-lawyer/) and go to flows.network to deploy your function. 

### Deploy the code on flow.network

1. Sign up for an account for deploying flows on [flows.network](https://flows.network/). It's free.
2. Click on the "Get Started" button and go to a new page, "My flows".
3. Click on the "Create a Flow" button to start deploying the ChatGPT Slack bot.
4. Authenticate the [flows.network](https://flows.network/) to access the `robo-lawyer` repo you just forked. Don't forget to choose "With Environment Variables", which we will configure the required parameters. Once done, click on the "Next" button.
<img width="772" alt="image" src="https://user-images.githubusercontent.com/45785633/224664082-5fa56d51-eb58-4ac8-8dea-d4c655a13791.png">

5. Fill in the required Environment Variables. In this example, we have three variables. One is `workspace`. Fill in the Slack workspace Name you want to connect here. The second one is `channel`. Fill in the Slack channel under the Slack workspace you just entered. The last one is `openai_key_name`. Fill in the name you want to name your OpenAI Key.

<img width="688" alt="image" src="https://user-images.githubusercontent.com/45785633/224656401-2a554e1b-8960-4995-ba69-f4e0637d0c63.png">


6. Name your flow, and click on "Turn on and Save" button to deploy the flow function.


### Configure SaaS integrations

After that, the flows.network will redirect you to the flow details page automatically. In the Flow details tab, we can set up SaaS integrations required by the flow.
<img width="1005" alt="image" src="https://user-images.githubusercontent.com/45785633/224656741-060cb9c4-02df-4683-b6ab-d49bf95465e1.png">

1. Click on the "Connect" button to authenticate your OpenAI account. You'll be redirected to a new page where you could copy and paste your OpenAI API key and then name the key. Note that the name you enter here should be the same as the name in the environment variables.

<img width="758" alt="image" src="https://user-images.githubusercontent.com/45785633/222973214-ecd052dc-72c2-4711-90ec-db1ec9d5f24e.png">

2. Click the "Connect" button to authenticate your Slack account. You'll be redirected to a new page where you must grant [flows.network](https://flows.network/) permission to install the `flows-network-integration` bot on a Slack worksapce. This workspace is the one you entered into the environment variables above.

That's all. As soon as the flow function's status becomes `ready`, the ChatGPT Slack App goes live. Go ahead and chat with ChatGPT by asking a legal question in the channel!

> [flows.network](https://flows.network/) is still in its early stages. We would love to hear your feedback!

## Others

If you want to build locally, make sure you have installed Rust and added `wasm32-wasi` traget.

```
cargo build --target wasm32-wasi --release
```
