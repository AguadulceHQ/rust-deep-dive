// mutable static variable
// static variables follow the SCREAMING_SNAKE_CASE convention
static mut ANSWER_TO_LIFE: u32 = 0;

static WELCOME: &str = "And the answer is...";

fn main() {
    println!("{WELCOME}");

    set_answer(3);

    unsafe {
        // this will work in a single thread but may cause issues in mutliple threads
        println!("{ANSWER_TO_LIFE}");
    }
}

fn set_answer(answer: u32) {
    // any code that read/writers a static variable should be within unsafe blocks
    unsafe {
        ANSWER_TO_LIFE = answer;
    }
}
