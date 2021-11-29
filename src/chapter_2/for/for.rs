fn main() {
    // for
    // The .. operator creates an iterator that generates numbers from a start number up to but not including an end number.
    for x in 0..5 {
        println!("{}", x);
    }
    // The ..= operator creates an iterator that generates numbers from a start number up to and including an end number.
    for x in 0..=5 {
        println!("{}", x);
    }
}