// when data is bound by the same name immutably, it's frozen
// this data cannot be modified until the immutable binding goes out of scope

fn main() {
    let mut mutable = 7i32;

    {
        // shadowing occurs here
        let mutable = mutable;
        // in this scope mutable is non mutable so we can't do
        //  mutable = 50; because is frozen
    }

    // the shadow is out of scope

    println!("We can access mutable again {}", {
        mutable += 1;
        mutable
    });
}
