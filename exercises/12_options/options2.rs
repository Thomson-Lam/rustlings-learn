fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);

        // TODO: Make this an if-let statement whose value is `Some`.
        // Since the optional_target is Option<&str> and is Some(target/ &str), the if let matches only if 
        // optional_target is Some(value) to bind the word to the inner value of &str 
        // if the optional_target was None this would not run, so if the optional_target is Some(&str) assert_eq!
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }

    // note: the pattern on the left side of the if let or while let must match the type 
    // and structure of the value on the right! It must fit the structure / Option being matched, Some or None! 

    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];

        for i in 1..=range {
            optional_integers.push(Some(i));
        }

        let mut cursor = range;

        // TODO: Make this a while-let statement. Remember that `Vec::pop()`
        // adds another layer of `Option`. You can do nested pattern matching
        // in if-let and while-let statements.
        // Outer Some() for popped Vec elems, inner Some() for Vec's entries returned as Option of Some or None
        while let Some(Some(integer)) = optional_integers.pop() { // wow the double some 'safety' lol; kind of redundant no?
            assert_eq!(integer, cursor);
            cursor -= 1;
        }

        assert_eq!(cursor, 0);
    }
}
