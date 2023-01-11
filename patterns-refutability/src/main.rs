fn main() {
    let project_number = 42; // x matches any value is irrefutable

    // we need an if because let Some(x) = won't cover all cases
    // this is a refutable pattern because it won't match None for example
    if let Some(x) = Some(project_number) {
        println!("The project ID is {}", x);
    }
}
