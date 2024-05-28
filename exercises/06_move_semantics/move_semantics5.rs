// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.


#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;// this is pointing to the memory location of x. (y) itself doesn't have the value but the mem location
    *y += 100;//this accesses the actual value pointed by y
    let z = &mut x;// this refers to the updated value of x after y's operation
    *z += 1000;
    assert_eq!(x, 1200);
}
