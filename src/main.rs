use yew_functional::{function_component, use_effect, use_state, use_effect_with_deps};
use yew::prelude::*;
use std::rc::Rc;
use weblog::console_log;
use yew_state::{SharedHandle, SharedState, SharedStateComponent};
use wasm_bindgen_futures::spawn_local;

async fn fetch(url: &str) -> anyhow::Result<String> {
    Ok(reqwest::get(url)
        .await?
        .text()
        .await?)
}

#[derive(Clone, PartialEq, Default, Debug)]
pub struct AppState {
    token: Option<String>,
    username: Option<String>,
}

#[function_component(Login)]
pub fn model(handle: &SharedHandle<AppState>) -> Html {
    let (username, set_username) = use_state(|| "".to_owned());
    let (password, set_password) = use_state(|| "".to_owned());

    /*let _onclick1 = {
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
        let username = Rc::clone(&username);
        let password = Rc::clone(&password);
        let handle = handle.clone();

        Callback::from(move |e| {
            let username = Rc::clone(&username);
            let password = Rc::clone(&password);
            let handle_callback = handle.reduce_callback_with(|state, (token, username)| {
                state.token = Some(token);
                state.username = Some(username);
                console_log!("set", format!("{:#?}", state))
            });
            spawn_local(async move {
                // pretend this response is a token
                // not using username, password in format! to provide a better error message
                // let resp = fetch(&format!("http://localhost:9090/api/hello/{}-{}", username, password)).await
                //     .unwrap();
                let resp = "bruh".to_string(); // we use bruh so we don't need the api

                handle_callback.emit((resp, (*username).clone()));
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

#[function_component(ShowData)]
fn show_data(handle: &SharedHandle<AppState>) -> Html {
    let (show, set_show) = use_state(|| false);
    html! {<>
        <button onclick=Callback::from(move |_| set_show(true))>{"show"}</button>
        {if *show {
            html! { <p>{ get_username(&handle) }</p> }
        } else {html! ()}}
    </>}
}

fn get_username(handle: &SharedHandle<AppState>) -> &str {
    &handle.state().username.as_ref().unwrap() // this panics pan
}

#[function_component(Application)]
fn application() -> Html {
    html! { <>
        <h1>{ "Hello world" }</h1>
        <SharedStateComponent<Login> />
        <ShowData />
    </>}
}

fn main() {
    yew::start_app::<Application>()
}
