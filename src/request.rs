use std::collections::HashMap;

pub type URLParams = HashMap<String, String>;

pub struct RequestConstructor {
    pub url_params: URLParams,
}

impl RequestConstructor {
    pub fn new() -> Self {
        Self {
            url_params: HashMap::new(),
        }
    }

    pub fn add_url_param(&mut self, key: &str, value: &str) -> () {
        self.url_params
            .insert(String::from(key), String::from(value));
    }
}
