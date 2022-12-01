use std::io::{self, Read, Result};
use libflate::zlib::{Encoder, Decoder};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str;
use substring::Substring;
extern crate base64;
extern crate table_extract;


#[derive(Debug)]
struct Next {
    action: String,
    link:String
}

fn main() -> Result<()>{

    let file = File::open("./samples/FE115 - VOIP Adsl e Fibra.htm")?;
    let mut buf_reader = BufReader::new(file);
    let mut html = String::new();
    buf_reader.read_to_string(&mut html)?;

    let table = table_extract::Table::find_first(&html).unwrap();
    for row in &table {

        let next_list:Vec<Next>=get_next(row.get("Next").unwrap_or("<next missing>"));

        println!("{:?}",next_list);

        //println!("{:?}",v);

        /*


        let mut link=String::from("");
        let mut action=String::from("");

        if v.len()>1 {
            action.push_str(&v[0]);
            link.push_str(v[1]);
        } else {
            link.push_str(v[0]);
        }

        //action.trim();
        //link.trim();
        
        // Program".chars().position(|c| c == 'g').unwrap()
        

        let start_index= match link.find(">"){
            Some(x)=> x,
            None => 0,   
         };

        let end_index= match link.find("</a>"){
            Some(x)=> x,
            None => link.len(),   
         };

        let _link=link.substring(start_index+1, end_index);

        println!("{}###{}",action.trim(),_link.trim());
        */
        /*
           <td class="next">
			TRUE&nbsp;
			<a href="#AUT_AMB_ESTAR[2837]_139784">AUT_AMB_ESTAR[2837]_139784</a>
            <br>FALSE&nbsp;
			<a href="#AUT_AMB_ESTAR[2837]_139778">AUT_AMB_ESTAR[2837]_139778</a></td>
           */
/*
        println!(
            "####ID########:{}\n#####ITEM#######:{}\n#####AZIONE#######:{}\n######NEXT######:{:?}\n",
            row.get("Id").unwrap_or("<id missing>"),
            row.get("Item").unwrap_or("<item missing>"),
            row.get("Azione").unwrap_or("<azione missing>"),
            action.action)
        );
 */
    }
     /*
    let file = File::open("./samples/sample.compress")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    //base64 decoder
    let buf = base64::decode(&mut contents).unwrap();

    println!("Base64 decoded:{:?}", buf);
    
    //zlib decoder

    //let mut encoder = Encoder::new(Vec::new()).unwrap();
    //io::copy(&mut buf, &mut encoder).unwrap();
    //io::copy(&mut &b"Hello!"[..], &mut encoder).unwrap();
    //let encoded_data = encoder.finish().into_result().unwrap();
    //println!("Encoded data: {:?}",&encoded_data[..]);

    //println!("Encoded data string: {}", &std::str::from_utf8(&encoded_data[..]).unwrap());
    //println!("Encoded data string: {}",&encoded_data[..]. 
    // Decoding
    //let mut decoder = Decoder::new(&buf[..]).unwrap();
    
    
    let mut decoder = Decoder::new(&buf[..]).unwrap();
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data).unwrap();

    

    //println!("Compressed {:?} and Uncompressed {:?}",&contents, &decoded_data);
    */
    Ok(())
}

    // txt1&nbsp;link1<br>txt2&nbsp;link2
fn get_next(td:&str) -> Vec<Next>{ 
        // TODO: action is an option
        
        let mut next_list:Vec<Next> = Vec::new();

        let v: Vec<&str> = td.split("<br>").collect();

        if v.len()==1 {
            let next=Next {
                action:String::from(""),
                link:get_link(&v[0].to_string()),
            };
            next_list.push(next);
        } else 
        {
            for c in v.iter() {
            let z:Vec<&str> =c.split("&nbsp;").collect();
                if z.len()==1 {
                    let next=Next {
                        action:String::from(""),
                        link:get_link(&v[0].to_string()),
                }; 
                } else {
                let next=Next {
                    action:string_clean(z[0].to_string()),
                    link:get_link(&z[1].to_string()),
               };
               next_list.push(next);
            }
        }
        }
        return next_list;
    }
    

fn string_clean(s:String) -> String{
      s.replace(&['\n','\t',' '][..],"")
 }

    fn get_link(link:&String) -> String{
        let start_index= match link.find(">"){
            Some(x)=> x,
            None => 0,   
         };

        let end_index= match link.find("</a>"){
            Some(x)=> x,
            None => link.len(),   
         };

        let _link=link.substring(start_index+1, end_index);
        return _link.to_string();
    }

    fn split_td(td: &str) -> Vec<&str>{

        let v: Vec<&str> = td.split("<br>").collect();

        return v;

        //<a href="#AUT_AMB_ESTAR[2837]_139776">AUT_AMB_ESTAR[2837]_139776</a>

    }
