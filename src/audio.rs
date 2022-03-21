use std::collections::HashMap;

use wasm_bindgen::JsValue;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use web_sys::HtmlAudioElement;

pub struct AudioManager {
    audio: Vec<HtmlAudioElement>,
    names: HashMap<String, usize>,
}

impl AudioManager {
    pub fn new() -> Self {
        Self {
            audio: vec![],
            names: HashMap::new(),
        }
    }

    pub fn insert(&mut self, path: String, img: HtmlAudioElement) {
        self.audio.push(img);
        self.names.insert(path, self.audio.len() - 1);
    }

    pub fn get(&self, path: &str) -> HtmlAudioElement {
        self.audio[self.names[path]].clone()
    }

    pub fn play(audio: HtmlAudioElement, set_loop: bool, force: bool, volume: f64) {
        if force {
            Self::stop(&audio);
        }
        spawn_local(async move {
            audio.set_volume(volume);
            audio.set_loop(set_loop);
            JsFuture::from(audio.play().unwrap()).await.unwrap_or(JsValue::UNDEFINED);
        });
    }

    pub fn play_name(&self, path: &str, set_loop: bool, force: bool, volume: f64) {
        let audio = self.get(path);
        Self::play(audio, set_loop, force, volume);
    }

    pub fn stop(audio: &HtmlAudioElement) {
        audio.pause().unwrap();
        audio.set_current_time(0.0);
    }
}
