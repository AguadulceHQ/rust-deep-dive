fn main() {
    let x = 5;
    let y = &x; // reference

    println!("x is equal to y in fact 👉 {} = {}", x, *y);
    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference

    let z = 5;
    let w = Box::new(z); // a Box that holds a reference to z

    println!("z is equal to w in fact 👉 {} = {}", z, *w);
    assert_eq!(5, z);
    assert_eq!(5, *w); // dereference
}
