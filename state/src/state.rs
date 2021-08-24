use crate::post::Post;
pub trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn add_text<'a>(&self, _text: &'a str) -> &'a str {
        ""
    }
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
}
