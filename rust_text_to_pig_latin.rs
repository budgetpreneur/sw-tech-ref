use std::io;
fn match_isvowel(input :&char) -> bool {
    if !input.is_alphabetic(){
        return false;
    }
    match input{
        'a'|'e'|'i'|'o'|'u' => true,
        'A'|'E'|'I'|'O'|'U' => true,
        _ => false
    }
}

fn match_isconsonant(input :&char) -> bool {
    if !input.is_alphabetic(){
        return false;
    }
    if match_isvowel(input) {
        false
    }
    else{
        true
    }
}

fn main() {
    println!("Enter the text to convert it to pig latin.");
    println!("NOTE: words ending non-alphanumeric will not be considered.");
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text)
        .expect("Failed to read input");

    let pig_latin_text = input_text.split_whitespace()
    .map(|w| { 
        let first_char = w.chars().nth(0).unwrap();
        if match_isvowel(&first_char) { 
            format!("{}-hay ",w) 
        }
        else if match_isconsonant(&first_char){
            format!("{}-{}ay ",&w[1..],first_char) 
        }
        else {
            format!("{} ",w) 
        } 
    })
    .collect::<String>();

    println!("Output = {}", pig_latin_text);
}
