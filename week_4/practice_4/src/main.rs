fn main() {
    let fullname = "Joshua Onyekachukwu Akpe";
    let department = "Computer Science";
    let uni = "Pan-Atlantic University";


    let mut school = "School of Science".to_string();
    // push string adds a given string to the end of another string
    school.push_str(" and Technology");

    println!("my name is {}",fullname );
    //length check
    println!("my full name has {} characters(including spaces)",fullname.len());
    println!("i am a student of the {} department",department );
    println!("{}", school);
    println!("{}",uni );
}
