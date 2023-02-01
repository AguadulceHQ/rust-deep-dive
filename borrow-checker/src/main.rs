#[derive(Debug)]
struct Items {}

fn main() {
    let item = vec![1, 2, 3];
    // item gets moved into this fn
    print_out_item(item);
    // main can't no longer access item
    // print_out_item(item);

    let item_ref = vec![1, 2, 3];
    print_out_item_by_ref(&item_ref);
    print_out_item_by_ref(&item_ref);

    let mut items = vec![Items {}, Items {}, Items {}];
    // immutable reference to a value
    let last_item = items.last();
    // here I have mutated the list so what last refers to now?
    // this requires a mutable reference but we already hold a immutable
    // items.pop();
    println!("The last item was {:?}", last_item);

    // now I can pop because I no longer use the immutable
    items.pop();
}

fn print_out_item(item: Vec<u32>) {
    for i in item {
        println!("{}", i);
    }
}

fn print_out_item_by_ref(item: &Vec<u32>) {
    for i in item {
        println!("{}", i);
    }
}
