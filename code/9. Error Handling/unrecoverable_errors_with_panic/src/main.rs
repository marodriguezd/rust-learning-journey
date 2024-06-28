/// RUST_BACKTRACE=1 cargo run
/// Put that into the console to execute with the full log

fn main() {
    explicit_panic(false);

    using_panic_backtrace(true);
}

fn explicit_panic(execute: bool) {
    if execute {
        panic!("crash and burn");
    }
}

fn using_panic_backtrace(execute: bool) {
    if execute {
        let v = vec![1, 2, 3];

        v[99];
    }
}