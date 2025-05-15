fn main() {
    // variables ust be initialized before they are used; compiler will throw an error if it cannot determine what value it holds.
    // this is a deliberate design choice to ensure all variables are initialized before use to avoid runtime errors.
    let x = 10;

    if x == 10 {
        println!("x is ten!");
    } else {
        println!("x is not ten!");
    }
}
