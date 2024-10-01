fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: Fix the Clippy lint.
    // if let
    if let Some(x) = option {
        res += x;
    }
    //or let else
    let Some(x) = option else { return };
    res += x;

    println!("{res}");
}
