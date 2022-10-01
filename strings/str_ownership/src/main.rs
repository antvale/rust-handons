use serde::{Serialize, Deserialize};
use std::cmp::Reverse;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct RecoveryPoint {
    id: String,
    file_name: String,
    valid: bool
}


fn main() -> std::io::Result<()>{
    let point1 = RecoveryPoint { 
        id:String::from("abc-123"), file_name:String::from("picture.png"), valid:true};
    
    let point2 = RecoveryPoint { 
            id:String::from("abc-124"), file_name:String::from("picture4.png"), valid:false};

    let mut list:Vec<RecoveryPoint>=Vec::new();

    list.push(point1);
    list.push(point2);

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&list).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);
    {
        let mut file = File::create("foo.txt")?;
        file.write_all(serialized.as_bytes())?;
    }
    {
        let file = File::options().open("foo.txt")?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        let deserialized: Vec<RecoveryPoint> = serde_json::from_str(&contents).unwrap();

        // Prints deserialized = Point { x: 1, y: 2 }
        println!("deserialized = {:?}", deserialized);
    }

    // Convert the JSON string back to a Point.
   

    
    let s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                                          // takes_and_gives_back, which also
                                                          // moves its return value into s3
    println!("{}",s1);
    println!("{}",s3);

    let mut a= String::from("The dog is the best ");
    a.push_str("friend of the human being!");

    let b=&a;
    println!("{}{}",a,b);

    let pair = (1, true);
    println!("pair is {:?}", pair);

    println!("the reversed pair is {:?}", pair);

    Ok(())

}

/**
 * The string slice and other fixed-sized types are pushed to stack and never
 * allocated in the heap. 
 * So for this kind of values there is no ownership matter to handle.
 */
fn str_ownership(){
    // immutable string slice pushed to stack
    let s = " The dog is the best friend of human being ";

    println!("{}",s);

    // s still lives after this assignment
    let t =s.trim();

    // both t and s are in the stack
    assert_eq!(s,t);

    println!("{}",t)

}

/**
 * String is a rust type used to contain mutable string value.
 * For this reason the value is allocated in the heap while the pointer, len and
 * capacity in the stack.
 * In order to avoid that two or more variables reference the same value in the heap through
 * two different pointers, that typically may bring to memory corruption, rust introduces 
 * the ownership pattern.
 * That's:
 * 1. each value in the heap has an owner
 * 2. just one onwer for a given value can exist at time
 * 3. every value out of the scope is dropped by compiler
 */
fn string_ownweship(){
    // immutable string
    let s=String::from("The dog runs faster a cat");

    println!("{}",s);

    // String doesn't implement copy so this statement is not valid
    // let t=s;

    // this assignment is correct because it assigns the reference and no a copy of the value
    let t = &s;

    // concat two string and assign the pointer to u
    let u= &format!("{}/{}",s,t);

    println!("{}",u);

}

fn mut_string_ownweship(){

    let mut s= String::from("The dog is less crafty than cat");

    //let mut t=s; //s moved to t so it becomes out of scope after this assigment
    //println!("{}",s); // you can't use s here

    let mut t = &s; //do not copy the value, instead the reference is assigned here
    println!("{}/{}",s,t);

    let mut u=format!("{}/{}",s,t);

    println!("{}",u);

}

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}