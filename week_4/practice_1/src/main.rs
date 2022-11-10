fn main() {
    let name = "Akpe Joshua";
    let uni:&str = "Pan-Atlantic University";
    let addr:&str = "KM 52 Lekki-Ekpe Expressway, Ibeju-Lekki, Lagos";
    println!("Name: {}",name);
    println!("University: {}, \nAddress: {}", uni,addr);


    let department:&'static str = "Computer Science";
    let school:&'static str = "School of Science and Technology";
    println!("Department: {}, \nSchool: {}",department,school);
}


