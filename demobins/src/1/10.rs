use blog::Post;

fn main() {
    let mut post = Post::new(); // 草稿

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review(); //预览
    assert_eq!("", post.content());

    post.reject(); //草稿状态

    post.request_review(); //预览
    assert_eq!("", post.content());

    post.approve(); // 审批1
    assert_eq!("", post.content());
    post.approve(); // 审批2
    assert_eq!("I ate a salad for lunch today", post.content());
    post.add_text("aaaaa");
}
