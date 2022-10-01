use serde::{Serialize, Deserialize};
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

    let mut list:Vec<RecoveryPoint>=Vec::new();

    let mut i=0;

    let max_index=100000;

   loop {

        let mut point = RecoveryPoint { 
            id:String::from("abc-123"), 
            file_name:String::from("picture.png"), 
            valid:true};
        point.id.push_str(&i.to_string().as_str());
        point.file_name.push_str(&i.to_string().as_str());
        list.push(point);

        i +=1;

        if i>max_index {break;}
   
    }

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&list).unwrap();

    // Prints serialized = {"x":1,"y":2}
    //println!("serialized = {}", serialized);
    {
        let mut file = File::create("foo.txt")?;
        file.write_all(serialized.as_bytes())?;
    }
    {
        let file = File::open("foo.txt")?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
        //let deserialized: Vec<RecoveryPoint> = serde_json::from_str(&contents).unwrap();

        // Prints deserialized = Point { x: 1, y: 2 }
        //println!("deserialized = {:?}", deserialized);
    }

    // Convert the JSON string back to a Point.

    Ok(())

}