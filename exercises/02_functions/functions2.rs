// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num: i32) { // function params must have explicit type annotations
    for i in 0..num { // 0..num is a range expression from 0-num inclusive (0 - num-1)
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
