fn main() {
    let a:i32 = 2;
    let b:i32 = 3;

    let mut res:i32;

    res = a & b;
    println!("(a & b) => {}",res );

    res = a | b;
    println!("(a | b) => {}",res );

    res = a ^ b;
    println!("(a ^ b) => {}",res );

    res = !b;
    println!("(!b) => {}",res );

    res = a << b;
    println!("(a << b) => {}",res );

    res = a >> b;
    println!("(a >> b) => {}",res );
}
