pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, s: &str) {
        if self.state.as_ref().unwrap().can_modify() {
            self.content.push_str(s);
        } else {
            panic!("Only modify on `Draft` status");
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(&self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }
    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject());
        }
    }
}
trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn can_modify(&self) -> bool {
        false
    }
    fn content<'a>(&self, _: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}
impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { count: 1 })
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn can_modify(&self) -> bool {
        true
    }
}
struct PendingReview {
    count: i32,
}
impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        if self.count != 2 {
            return Box::new(PendingReview {
                count: self.count + 1,
            });
        }
        Box::new(Published {})
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}
struct Published {}
impl State for Published {
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

// 增加 reject 方法将博文的状态从 PendingReview 变回 Draft。
// 在将状态变为 Published 之前要求两次 approve 调用。
// 只允许博文处于 Draft 状态时增加文本内容。提示：让状态对象负责内容可能发生什么改变，但不负责修改 Post。

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
