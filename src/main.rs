#![allow(non_snake_case)]

use std::{collections::HashMap, error::Error};

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use rand::{thread_rng, Rng};
use story_lib::sound_effect::SoundEffect;

pub static SOUND_EFFECTS: GlobalSignal<HashMap<String, Vec<u8>>> =
    Signal::global(|| HashMap::new());
fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");
    launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️
    let sound: Resource<Result<(), Box<dyn Error>>> = use_resource(move || async move {
        let res = reqwest::get("http://localhost:8080/umm.wav").await?;
        info!("umm.wav load");
        let bytes = res.bytes().await?.to_vec();
        info!("bytes to Vec<u8>");

        SOUND_EFFECTS.write().insert("umm".to_string(), bytes);
        info!("save");
        Ok(())
    });

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        match &*sound.read(){
            Some(Ok(())) => rsx!{
                Test{}
            },
            Some(Err(e)) => rsx! {
                p {"Loading failed, {e}"}
            },
            None => rsx!{p{"Loading..."}}
        }
    }
}

#[component]
fn Test() -> Element {
    let click = move |_| {
        let mut rng = thread_rng();
        let sound = SoundEffect::new(SOUND_EFFECTS().get("umm").cloned().unwrap()).unwrap();
        let sound = sound.is_rev(true).speed(rng.gen_range(0.5..2.));

        sound.play().unwrap();
    };
    rsx! {
        button {
            onclick: click,
            "test"
        }
    }
}
