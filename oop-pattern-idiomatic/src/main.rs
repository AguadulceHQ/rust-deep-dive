use oop_pattern_idiomatic::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("This is the Rust way of doing things 🤘");
    // shadow assignment to save the returned instances
    let post = post.request_review();
    let post = post.approve();

    assert_eq!("This is the Rust way of doing things 🤘", post.content());
}
