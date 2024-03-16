pub struct ReponseConstructor {
    status: String,
    content_type: String,
    content: String,
}

impl ReponseConstructor {
    pub fn new() -> Self {
        Self {
            status: String::from("200 OK"),
            content: String::new(),
            content_type: String::from("text/plain"),
        }
    }

    pub fn set_status(&mut self, status: &str) -> &mut Self {
        self.status = status.to_string();

        self
    }

    pub fn set_content(&mut self, content: &str) -> &mut Self {
        self.content = content.to_string();

        self
    }

    pub fn set_content_type(&mut self, content_type: &str) -> &mut Self {
        self.content_type = content_type.to_string();

        self
    }

    pub fn build(&self) -> Vec<u8> {
        format!(
            "HTTP/1.1 {}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status,
            self.content_type,
            self.content.len(),
            self.content
        )
        .into_bytes()
    }
}
