use yew_functional::{function_component, use_effect, use_state, use_effect_with_deps};
use yew::prelude::*;
use std::rc::Rc;
use weblog::console_log;
use yew_state::{SharedHandle, SharedState};
use wasm_bindgen_futures::spawn_local;

async fn fetch(url: &str) -> anyhow::Result<String> {
    Ok(reqwest::get(url)
        .await?
        .text()
        .await?)
}

#[derive(Clone, PartialEq, Default)]
pub struct AppState {
    token: Option<String>,
}

#[function_component(Login)]
pub fn model(handle: &SharedHandle<AppState>) -> Html {
    let (username, set_username) = use_state(|| "".to_owned());
    let (password, set_password) = use_state(|| "".to_owned());

   /* let _onclick1 = {
        let username = Rc::clone(&username);
        let password = Rc::clone(&password);

        handle.reduce_callback(|state| {
            spawn_local(async move {
                // pretend this response is a token
                let resp = fetch(&format!("http:://localhost:9090/api/hello/{}-{}", username, password)).await
                    .unwrap();

                // in a struct component, we'd send a message with this data and update the state there
                // but here, we don't have messages or link so we need to mutate the state here like
                state.token = Some(resp);
                // but when we try to do that, we realize that `state` doesn't live long enough to passed into
                // spawn_local and therefore we can't modify it.
                // Error message: https://pastify-app.web.app/show/qSZxhX59IYSSzp6it88G
            })
        })
    };*/

    let onclick = {
        let handle = handle.clone();

        Callback::from(|e| {
            spawn_local(async move {
                // pretend this response is a token
                // not using username, password in format! to provide a better error message
                let resp = fetch(&format!("http:://localhost:9090/api/hello/test")).await
                    .unwrap();

                handle.reduce(|s| s.token = Some(resp))
                // here we don't get any issues with lifetimes but a clone is required
                // but here we have issues with Fn vs FnOnce
                // Error message: https://pastify-app.web.app/show/zgZT6yAKFyHjhu2Uc7uD
                // There may be a way to resolve this but I'm not aware of it
            })
        })
    };

    html! {<>
        <input
            type="text"
            value=username
            placeholder="Username"
            oninput=Callback::from(move |e: InputData| set_username(e.value))
        />

        <input
            type="password"
            value=password
            placeholder="Password"
            oninput=Callback::from(move |e: InputData| set_password(e.value))
        />

        <button onclick=onclick>{ "Login" }</button>
    </>}
}

#[function_component(Application)]
fn application() -> Html {
    html! { <>
        <h1>{ "Hello world" }</h1>
        <Login />
    </>}
}

fn main() {
    yew::start_app::<Application>()
}
