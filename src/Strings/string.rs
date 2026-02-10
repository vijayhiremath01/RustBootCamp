
// Taking the first word from the string 
fn main(){
    let name = String::from("Vijay Hiremath");
    println!("{}" , name);
    
    let first_word = get_first_word(name);
    println!("{}" , first_word);
}

// Function to get the first word from the string 
fn get_first_word(name : String) -> String {
     let mut ans = String::from("");
     for char in name.chars(){
        ans.push_str(char.to_string().as_str());
        if char == ' ' {
            break ; 
        }
     }
     return ans ; 
}       