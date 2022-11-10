fn main() {
    let a:i32 = 10;
    let b:i32 =20;

    println!("value of A: {}",a);
    println!("value of B: {}",b);

    let mut res = a>b;
    println!("a greater than b: {}",res );

    res =a < b;
    println!("a less than b: {}",res );

    res = a>=b;
    println!("a graeter than or equal to b: {}",res );

    res = a <= b;
    println!("a less than or equal to b: {}",res);

    res = a==b;
    println!("a is equal to b: {}",res );

    res = a!=b;
    println!("a is not equal to b {}",res);
}
