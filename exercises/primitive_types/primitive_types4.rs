// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];
    let b = [2, 3, 4];

    let nice_slice = &a[1..b.len()+1];

    assert_eq!(b, nice_slice)
}

