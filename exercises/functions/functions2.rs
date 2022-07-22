// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.


fn main() {
    call_me(3);
}

fn call_me(num: isize) {
    for i in num - 5..num +1 {
        println!("Ring! Call number {}", i + 1);
    }
}
