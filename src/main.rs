// use std::fmt::format;
mod variable;
fn main() {
    let mut string = String::from("hello world");
    let  str = "welcome to rust";
    let string_to_str = &str.to_string();
    let array_concat_string = ["hello", "world"].concat();

    let mut sally =String::from("my name is salman");
    let add_to_sally = &sally.push_str("  and i love sally ");
    let add_char_to_sally = &sally.push('X');


    let _add_to_string= &string.push_str(" something else ");
    let _add_another_char = &string.push('d');

    //format macro
    let combine_string = format!("{} {} {}", string, str, "This is me");
    println!("{}", combine_string);
    
    println!("{}", sally);
    println!("{}", string);
    println!("{}", string_to_str);
    println!("{}", array_concat_string);
    // println!("{}", combined_string);
    println!("{}", str);

    // println!("{:?}", add_to_string);
    //  println!("{}", add_another_char);
 
    let slice = &string[0..4];
    let slice_fourth = &string[0..=4];

    println!("{}", slice);
    println!("{}", slice_fourth);
    variable::run;
    
}



