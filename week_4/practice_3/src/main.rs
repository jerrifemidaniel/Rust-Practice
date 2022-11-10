fn main() {
    let name1 = "Ayomide Adesokan";
    println!("my name is {}",name1 );

    //find and replace 
    let name2 = name1.replace("Ayomide","Adebare");
    println!("you can also call me {}",name2 );
    let faculty = "faculty of science and technology";

    //find and replace 
    let school = faculty.replace("faculty","School");
    println!("I am a student of the {}",school );
}
