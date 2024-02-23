fn concatenate_strings(s1: &str, s2: &str) -> String 
{
    let mut result = String::new();
    result.push_str(s1);
    result.push_str(s2);
    result
}

fn main() 
{
    let string1 = String::from("Hi, I'm Hammad Qaiser. ");
    let string2 = String::from("Welcome to Rust Bootcamp in Pakistan.");
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("Concatenated string: {}", concatenated_string);
    
    /* The variables string1 and string2 are still valid and can be used here.
    It means not violating any ownership reules. Borrowing and reference rules are valid */
    println!("string1 is {}", string1);
    println!("string2 is {}", string2);
}
