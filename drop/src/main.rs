// the Drop trait is included in the prelude so there is no need of bringing into scope.
#[derive(Debug)]
struct TaskPointer {
    data: String,
}

impl Drop for TaskPointer {
    fn drop(&mut self) {
        // our implementation says to print when TaskPointer types go out of scope
        println!("The task `{}` is done ✅", self.data);
    }
}

fn main() {
    let task_one = TaskPointer {
        data: String::from("Upgrade Polkadot's version to latest"),
    };
    let task_two = TaskPointer {
        data: String::from("Run cargo audit and ensure that all warnings are resolved"), // this gets dropped first
    };

    println!(
        "We access our Tasks before going out of scope. The task list is: 1. {:?} 2. {:?}",
        task_one, task_two
    );
    println!("Tasks have been created. But we are going out of scope after main is done let's see who gets de-allocated first, the compiler will call our implementation of drop before de-allocating the memory.");
}
