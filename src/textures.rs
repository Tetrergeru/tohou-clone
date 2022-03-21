use std::collections::HashMap;

use web_sys::HtmlImageElement;

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
}
