use std::fs::File;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;


fn main() -> std::io::Result<()>{
    println!("Start app");
    
    let file_name="hello.txt";
    append_to_file(file_name,"pluto")?;
    /*
    if Path::new(file_name).exists() {
        std::fs::remove_file(file_name).unwrap();
      }
    */
    //println!("Deleted file {}",file_name);

    //File::create("hello.txt").unwrap();
    /*
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap();
    
    if let Err(e) = writeln!(file, "{}", "pippo") {
            eprintln!("Couldn't write to file: {}", e);
        }
    */
    /*
    let file = File::open("hello.txt").unwrap_or_else(|error|{
        if error.kind()==ErrorKind::NotFound {
                return File::create("hello.txt").unwrap_or_else(|error|{
                    panic!("Error creating the file:{:?}",error);
        })
        } else {
            panic!("Error opening the file {:?}",error);
        }
    });
    */

/*
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) =>  Err(()),
    };
*/
/*
    let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents)?;
*/
    /*
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => file,
    };
    */

    println!("Continue...");

    Ok(())
}

/**
 * Append a line to file. If the file doesn't exist a new file is created.
 *
 */
fn append_to_file(file_name: &str, line: &str) -> std::io::Result<()>{

    let is_exist=Path::new(file_name).exists();

    let mut file = OpenOptions::new()
    .create_new(!is_exist) 
    .write(true)
    .append(true)
    .open(file_name)?;

    writeln!(file, "{}", line)?;

    Ok(())
}