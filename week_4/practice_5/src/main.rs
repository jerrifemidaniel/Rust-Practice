fn main() {
    let schoolname = " Pan-Atlantic University ";
    println!();
    println!("name: {}",schoolname );
    println!();
    println!("before trim");
    println!("length is {}",schoolname.len());
    println!();
    println!("after trim");
    println!("length is {}",schoolname.trim().len());
    //trim removes unneccessary spaces
}
