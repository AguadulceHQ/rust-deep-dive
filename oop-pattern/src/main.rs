use oop_pattern::Post;

fn main() {
    // every post starts as a draft
    let mut post = Post::new();

    // we want to be able to add text
    post.add_text("This is how you print something in Rust");
    // if it wasn't approved we should not get content through its interface
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("This is how you print something in Rust", post.content());
}
