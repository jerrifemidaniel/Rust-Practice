fn main() {
    let n1 = "electrical".to_string();
    let n2 = " electronic".to_string();
    let n3 = " engineering".to_string();
    let n4 = n1 + &n2 + &n3; //& has to be in front of all but the first one


    println!("\nthe {} is informed by the aspiration to train 
        electrical/electronic engineering professionals 
        in the areas of design, building, and maintenance of 
        electrical control systems.",n4);
    let w1 = "computer".to_string();
    let w2 = " science".to_string();
    let w3 = w1 + &w2;
    println!();
    println!("{} is aimed at developing competent, creative,
     innovative, entrepreneural, and ethically-minded persons,
     capable of creating value in the diverse fields of computer science",w3);
}
