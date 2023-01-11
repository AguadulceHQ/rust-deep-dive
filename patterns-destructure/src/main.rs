fn main() {
    // match deconstructing structs

    struct Feedback {
        communication: i32,
        overall: i32,
    }

    let f1 = Feedback {
        communication: 3,
        overall: 4,
    };

    let Feedback {
        communication: c,
        overall: o,
    } = f1; // match variables c and o to the values of fields in f1 struct

    let Feedback {
        communication,
        overall,
    } = f1; // shorthand notation give the frequent use case

    assert_eq!(3, c,);
    assert_eq!(3, communication);
    assert_eq!(4, o);
    assert_eq!(4, overall);
}
