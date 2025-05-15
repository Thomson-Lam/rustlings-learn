fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    // TODO: Fix the compiler errors only by reordering the lines in the test.
    // Don't add, change or remove any line.
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    /*
    In Rust, a mutable reference (&mut) is dropped (released) as soon as it goes out of scope or is no longer used. 
    This ensures that you can safely create another mutable reference to the same value after the first one is no longer active.
    So reordering the lines makes it such that there is no redundant reference and data race.
     */
    }
}
