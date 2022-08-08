pub struct Post {
    content: String,
}

pub struct DraftPost {
    content: String,
}

pub struct PendingPost {
    content: String,
}
pub struct InAprobationPost {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: "".to_string(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingPost {
        PendingPost {
            content: self.content,
        }
    }
}

impl PendingPost {
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
    pub fn approve(self) -> InAprobationPost {
        InAprobationPost {
            content: self.content,
        }
    }
}

impl InAprobationPost {
    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
    pub fn approve(self) -> Post {
        Post {
            content: self.content,
        }
    }
}
