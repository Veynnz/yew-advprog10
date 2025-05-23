use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="bg-gradient-to-r from-violet-200 to-pink-200 flex w-screen h-screen items-center justify-center">
            <div class="bg-white/80 backdrop-blur-sm p-8 rounded-2xl shadow-xl max-w-md w-full">
                <div class="text-center mb-8">
                    <h1 class="text-3xl font-bold text-violet-800 mb-2">{"Welcome to YewChat! 👋"}</h1>
                    <p class="text-violet-600">{"Connect with friends in real-time"}</p>
                </div>
                <form class="space-y-4">
                    <div class="relative">
                        <input 
                            {oninput} 
                            class="w-full px-4 py-3 rounded-xl border-2 border-violet-200 focus:border-violet-400 focus:outline-none transition-all pl-10" 
                            placeholder="Enter your username"
                        />
                        <span class="absolute left-3 top-3.5 text-violet-400">{"👤"}</span>
                    </div>
                    <Link<Route> to={Route::Chat}>
                        <button 
                            {onclick} 
                            disabled={username.len()<1} 
                            class="w-full px-4 py-3 rounded-xl bg-violet-600 hover:bg-violet-700 text-white font-medium shadow-lg hover:shadow-violet-400/50 transition-all disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            {"Start Chatting! 🚀"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}