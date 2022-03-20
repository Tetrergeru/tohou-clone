use std::collections::HashMap;

use futures::channel::oneshot;
use gloo_net::http::Request;
use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlImageElement, Url};

pub struct TextureManager {
    textures: Vec<HtmlImageElement>,
    names: HashMap<String, usize>,
}

impl TextureManager {
    pub fn new() -> Self {
        Self {
            textures: vec![],
            names: HashMap::new(),
        }
    }

    pub fn insert(&mut self, path: String, img: HtmlImageElement) {
        self.textures.push(img);
        self.names.insert(path, self.textures.len() - 1);
    }

    pub fn get(&self, path: &str) -> &'_ HtmlImageElement {
        &self.textures[self.names[path]]
    }

    pub async fn download(path: &str) -> HtmlImageElement {
        let resp = Request::get(path).send().await.unwrap();
        let blob = JsFuture::from(resp.as_raw().blob().unwrap()).await.unwrap();

        let url = Url::create_object_url_with_blob(&blob.unchecked_into()).unwrap();
        let image = HtmlImageElement::new().unwrap();

        let (send, recv) = oneshot::channel();

        let on_load_closure = Closure::once(Box::new(move || {
            send.send(()).unwrap();
        }) as Box<dyn FnOnce()>);
        image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
        on_load_closure.forget();

        let cloned_path = path.to_string();
        let on_error_closure = Closure::wrap(Box::new(move || {
            panic!("image {} loading failed", cloned_path);
        }) as Box<dyn FnMut()>);
        image.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
        on_error_closure.forget();

        image.set_src(&url);

        recv.await.unwrap();

        image
    }
}
