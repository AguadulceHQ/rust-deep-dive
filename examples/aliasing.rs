// type statement can be used to give a new name to an existing type
// types follow UpperCamelCase names expect primitive types
// main use for aliases is to reduce boilerplace e.g. IoResult<T> is an alias for Result<T, IOError>

type NanoSecond = u64;
type Inch = u64;
type U64 = u64;

fn main() {
    // NanoSecond = Inch = U64 = u64
    let nanoseconds: NanoSecond = 5 as U64;
    let inches: Inch = 2 as U64;

    println!(
        "{} nanoseconds + {} inches = {} unit",
        nanoseconds,
        inches,
        nanoseconds + inches
    );
}
