fn main() {
    let mut s = String::from("hello world");
    let word = first_word_impl1(&s);
    println!("s is {}", s);
    println!("word is {}", word);
    let first_word_1 = &s[0..word];         //borrow slice of s
    println!("first_word is {}", first_word_1); 
    let first_word_2 = first_word_impl2(&s);
    println!("Another implementaton of first word");
    println!("first word is still {}", first_word_2);
    s.clear();
}

fn first_word_impl1(s: &String) -> usize {
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

fn first_word_impl2(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }
    &s
}
