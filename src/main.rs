fn main(){

    // Numbers 
    let x: i8 = -12 ;
    println!("x : {} " , x);
    println!("{}" , std::mem::size_of_val(&x));
   
   // Initialise the String in RUST 
   let name = String::from("Vijay Hiremath");
   println!("{}" , name );

   let char1 = name.chars().nth(78);
   let char2 = name.chars().nth(4);
   // Printing the character at some index 
    match char1 {
        Some(name) => println!("{}" , name),
        None => println!("Nothing is there in that index ! "),
    }
    match char2 {
        Some(name) => println!("{}" , name),
        None => print!("Nothing is there in that index !")
    }

}

// fn main(){

//     let male_info = false ;
//     let female_info = true;
//     let dont_know_info = true ; 

//     if male_info {
//         print!("Male");
//     }
//     else if female_info {
//         print!("Female");
//     } 
//     else if dont_know_info {
//         print!("Dont know ! ");
//     }
// }

