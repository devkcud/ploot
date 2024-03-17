use std::collections::HashMap;

pub type URLParams = HashMap<String, String>;

pub struct RequestConstructor {
    pub url_params: URLParams,
    pub group: String,
}

impl RequestConstructor {
    pub fn new() -> Self {
        Self {
            url_params: HashMap::new(),
            group: String::new(),
        }
    }
}
