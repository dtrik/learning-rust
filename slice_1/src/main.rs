fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    println!("s is {}", s);
    println!("word is {}", word);
    let first_word = &s[0..word];               //borrow slice of s
    println!("first_word is {}", first_word); 
    s.clear();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();                   //convert s to bytes
    //iter returns each element in collection, enumerate returns each element as
    //part of tuple = (index, reference)
    for (i, &item) in bytes.iter().enumerate() {       
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

