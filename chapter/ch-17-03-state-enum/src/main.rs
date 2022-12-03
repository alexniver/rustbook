use ch_17_03_state_enum::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("this is a post");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("this is a post", post.content());
}
