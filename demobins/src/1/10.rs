use std::ops::Add;
pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Post {
    pub title: String,   // 标题
    pub author: String,  // 作者
    pub content: String, // 内容
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

fn some_article(a: impl Summary) {
    println!("{}", a.summarize());
}
fn main() {
    let weibo = Weibo {
        username: "haha".to_string(),
        content: "这是一篇文章".to_string(),
    };
    some_article(weibo);
}
