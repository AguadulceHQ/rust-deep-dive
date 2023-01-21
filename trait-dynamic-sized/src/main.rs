fn main() {
    // the compiler doesn't support that. These strings require different space in memory so they can't use the same type
    //    let s1: str = "Hello";
    //    let s2: str = "Aguadulce";

    // these work because this are slices pointer that have the reference to the actual string
    // and length to know until where in memory the string is stored
    // the length of a reference that holds a pointer + a number for length is known to the compiler as those are both fixed types
    let s1: &str = "Hello";
    let s2: &str = "Aguadulce";

    println!("{} {}", s1, s2);
}
