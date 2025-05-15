// TODO: Change the line below to fix the compiler error.
const NUMBER: i32 = 3;
// consts need an explicit type declaration because they are evaluated at compile time and not run time type inferred.
// consts do not benefit from type inference because they are not mutable at all and not scoped.

fn main() {
    println!("Number: {NUMBER}");
}
