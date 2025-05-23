use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

use crate::{User, services::websocket::WebsocketService};
use crate::services::event_bus::EventBus;

pub enum Msg {
    HandleMsg(String),
    SubmitMessage,
}

#[derive(Deserialize)]
struct MessageData {
    from: String,
    message: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MsgTypes {
    Users,
    Register,
    Message,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebSocketMessage {
    message_type: MsgTypes,
    data_array: Option<Vec<String>>,
    data: Option<String>,
}

#[derive(Clone)]
struct UserProfile {
    name: String,
    avatar: String,
}

pub struct Chat {
    users: Vec<UserProfile>,
    chat_input: NodeRef,
    wss: WebsocketService,
    messages: Vec<MessageData>,
    _producer: Box<dyn Bridge<EventBus>>,
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let (user, _) = ctx
            .link()
            .context::<User>(Callback::noop())
            .expect("context to be set");
        let wss = WebsocketService::new();
        let username = user.username.borrow().clone();

        let message = WebSocketMessage {
            message_type: MsgTypes::Register,
            data: Some(username.to_string()),
            data_array: None,
        };

        if let Ok(_) = wss
            .tx
            .clone()
            .try_send(serde_json::to_string(&message).unwrap())
        {
            log::debug!("message sent successfully");
        }

        Self {
            users: vec![],
            messages: vec![],
            chat_input: NodeRef::default(),
            wss,
            _producer: EventBus::bridge(ctx.link().callback(Msg::HandleMsg)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::HandleMsg(s) => {
                let msg: WebSocketMessage = serde_json::from_str(&s).unwrap();
                match msg.message_type {
                    MsgTypes::Users => {
                        let users_from_message = msg.data_array.unwrap_or_default();
                        self.users = users_from_message
                            .iter()
                            .map(|u| UserProfile {
                                name: u.into(),
                                avatar: format!(
                                    "https://avatars.dicebear.com/api/adventurer-neutral/{}.svg",
                                    u
                                )
                                .into(),
                            })
                            .collect();
                        return true;
                    }
                    MsgTypes::Message => {
                        let message_data: MessageData =
                            serde_json::from_str(&msg.data.unwrap()).unwrap();
                        self.messages.push(message_data);
                        return true;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            Msg::SubmitMessage => {
                let input = self.chat_input.cast::<HtmlInputElement>();
                if let Some(input) = input {
                    //log::debug!("got input: {:?}", input.value());
                    let message = WebSocketMessage {
                        message_type: MsgTypes::Message,
                        data: Some(input.value()),
                        data_array: None,
                    };
                    if let Err(e) = self
                        .wss
                        .tx
                        .clone()
                        .try_send(serde_json::to_string(&message).unwrap())
                    {
                        log::debug!("error sending to channel: {:?}", e);
                    }
                    input.set_value("");
                };
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let submit = ctx.link().callback(|_| Msg::SubmitMessage);
        html! {
            <div class="flex w-screen bg-gradient-to-r from-violet-200 to-pink-200">
                // Sidebar with users
                <div class="flex-none w-64 h-screen bg-white/80 backdrop-blur-sm shadow-xl">
                    <div class="text-2xl p-4 font-bold text-violet-800 flex items-center">
                        <span class="mr-2">{"👥"}</span>
                        {"Online Friends"}
                    </div>
                    {
                        self.users.clone().iter().map(|u| {
                            html!{
                                <div class="flex m-3 bg-white/90 rounded-xl p-3 shadow-md hover:shadow-lg transition-all">
                                    <div class="relative">
                                        <img class="w-12 h-12 rounded-full border-2 border-violet-400" src={u.avatar.clone()} alt="avatar"/>
                                        <div class="absolute bottom-0 right-0 w-3 h-3 bg-green-400 rounded-full border-2 border-white"></div>
                                    </div>
                                    <div class="flex-grow p-2">
                                        <div class="font-medium text-violet-900">{u.name.clone()}</div>
                                        <div class="text-xs text-violet-600">{"✨ Online"}</div>
                                    </div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>

                // Main chat area
                <div class="grow h-screen flex flex-col bg-white/60 backdrop-blur-sm">
                    <div class="w-full h-16 border-b-2 border-violet-200 flex items-center px-6">
                        <div class="text-2xl font-bold text-violet-800">{"💬 YewChat"}</div>
                        <div class="ml-4 text-sm text-violet-600">{"Share your thoughts with friends!"}</div>
                    </div>
                    
                    // Messages area
                    <div class="w-full grow overflow-auto p-4">
                        {
                            self.messages.iter().map(|m| {
                                let user = self.users.iter().find(|u| u.name == m.from).unwrap();
                                html!{
                                    <div class="flex items-start mb-4">
                                        <img class="w-8 h-8 rounded-full mr-3" src={user.avatar.clone()} alt="avatar"/>
                                        <div class="max-w-xl">
                                            <div class="text-sm font-medium text-violet-900 mb-1">
                                                {m.from.clone()}
                                            </div>
                                            <div class="bg-white rounded-2xl rounded-tl-none px-4 py-2 shadow-md">
                                                if m.message.ends_with(".gif") {
                                                    <img class="rounded-lg max-w-sm" src={m.message.clone()}/>
                                                } else {
                                                    <span class="text-gray-800">{m.message.clone()}</span>
                                                }
                                            </div>
                                        </div>
                                    </div>
                                }
                            }).collect::<Html>()
                        }
                    </div>

                    // Input area
                    <div class="w-full h-20 flex px-6 items-center bg-white/80 border-t border-violet-200">
                        <input 
                            ref={self.chat_input.clone()} 
                            type="text" 
                            placeholder="Type your message here... 💭" 
                            class="block w-full py-3 px-4 bg-white rounded-xl outline-none focus:ring-2 focus:ring-violet-400 transition-all shadow-sm" 
                            name="message" 
                            required=true 
                        />
                        <button 
                            onclick={submit} 
                            class="ml-4 p-3 bg-violet-600 hover:bg-violet-700 w-12 h-12 rounded-xl flex justify-center items-center shadow-lg hover:shadow-violet-400/50 transition-all"
                        >
                            <svg viewBox="0 0 24 24" class="w-6 h-6 fill-white transform rotate-45">
                                <path d="M2.01 21L23 12 2.01 3 2 10l15 2-15 2z"/>
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}