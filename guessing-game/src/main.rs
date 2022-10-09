use std::{io::stdin, cmp::Ordering};
use std::collections::HashMap;
use rand::Rng;

fn main() {
    //string_by_example();
    //count_words();

    let mut table = HashMap::new();

    table.insert("key1", "value1");

    println!("{:?}",&table);

    

    for (key,value) in &table {
        println!("Get value {} from key {}",key,value);
    }

    println!("{}",table.get(&"key1").unwrap());

}

fn guess_game(){
    println!("Guess the number!");
   
    let secret_number= rand::thread_rng().gen_range(1..=100);
    
    println!("The secret number is: {secret_number}");


    loop {
        println!("Please input your guess:");
        let mut guess= String::new();

        stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32=match guess.trim().parse(){
            Ok(num) => num,
            Err(_)  => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => print!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn string_by_example() {
    //string slice
    let s = "' the quick brown fox jumps over the lazy dog'";

    let mut t= s;
    println!("Words in revese");
    for word in s.split_whitespace().rev(){
        println!(">{}",word);
    }

    t=s.trim_start();

    println!("print:{}",s.trim_start());

    let mut chars: Vec<char>= s.chars().collect();
    chars.sort();
    chars.dedup();

    println!("print:{}",t);

    // Empty and growable String
    let mut string = String::new();

    for c in chars {
        string.push(c);
        string.push_str(", ");
    }
    println!("{}",string);
    let chars_to_trim= &[' ',','];
    let trimmed_str = string.trim_end_matches(chars_to_trim);
    println!("Used characters: {}", trimmed_str);

    // Heap allocate a string
    let alice = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let bob: String = alice.replace("dog", "cat");

    println!("Alice says: {}", alice);
    println!("Bob says: {}", bob);

}


fn count_words() {
 // word => count
  let mut words: HashMap<String, u32> = HashMap::new();

  let mut word_list:Vec<String> = Vec::new();

  loop {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    //println!("{}",input);

    //println!("{:?}",input.split_whitespace());

    let mut split=input.split_whitespace();
/*
    while let Some(word)=split.next() {
        //println!("{}",word.trim());
        // word_list.push(word.to_string());
        words.insert(word.to_string(),1);
    }
  */

    for word in input.split_whitespace(){

        let count = match words.get(word) {
            None => 0,
            Some(count) => *count,
          };
          words.insert(word.to_string(), count + 1);

    }
    

    println!("{:?}", words);
  }
}