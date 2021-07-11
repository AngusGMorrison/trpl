pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost::new(self.content)
    }
}

pub struct PendingReviewPost {
    content: String,
    approval_count: u8,
}

const APPROVALS_REQUIRED: u8 = 2;

impl PendingReviewPost {
    fn new(content: String) -> PendingReviewPost {
        PendingReviewPost {
            content,
            approval_count: 0,
        }
    }

    pub fn approve(&mut self) {
        self.approval_count += 1;
    }

    pub fn publish(self) -> Result<Post, String> {
        if self.approval_count < APPROVALS_REQUIRED {
            let msg = format!(
                "PendingReviewPost must have at least 2 approvals to publish. Current approvals: {}",
                self.approval_count
            );
            Err(msg)
        } else {
            Ok(Post {
                content: self.content,
            })
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
    }
}
