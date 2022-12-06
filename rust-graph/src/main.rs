use std::io::{self, Read, Result};
use libflate::zlib::{Encoder, Decoder};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str;
use substring::Substring;
extern crate table_extract;

#[macro_use]
extern crate simple_excel_writer as excel;

use excel::*;

#[derive(Debug)]
struct Next {
    action: String,
    link:String
}

fn main() -> Result<()>{

    let mut filename = String::from("./samples/FE251 - FO - Borchia GSM.htm");

    let file = File::open(&filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut html = String::new();
    buf_reader.read_to_string(&mut html)?;

    filename.push_str(".xlsx");

    let mut wb = Workbook::create(&filename.as_str());
    
    let mut nodes = wb.create_sheet("Nodes");
    let mut edges = wb.create_sheet("Edges");

    nodes.add_column(Column { width: 30.0 });
    nodes.add_column(Column { width: 30.0 });
    nodes.add_column(Column { width: 30.0 });

    edges.add_column(Column { width: 30.0 });
    edges.add_column(Column { width: 30.0 });
    edges.add_column(Column { width: 30.0 });

    let table = table_extract::Table::find_first(&html).unwrap();
    
    wb.write_sheet(&mut nodes, |sheet_writer| {
        let sw = sheet_writer;
        sw.append_row(row!["ID", "Name","Role"])?;
    
        //"Target","Source","Type","Label"

    let mut index=0;
   
    for row in &table {
        //if index > 1 {break;}
        let id=row.get("Id").unwrap_or("<id missing>");
        let item=row.get("Item").unwrap_or("<item missing>");
        let azione=row.get("Azione").unwrap_or("<azione missing>");
        //let mut next_list:Vec<Next>=get_next(row.get("Next").unwrap_or("<next missing>"));
        sw.append_row(row![id, azione,item])?;
        /*
        for next in next_list.iter() {
        }
        */
        index=index+1;
       // println!("{},{},{},{:?}",id,item,azione,next_list);
    }
    sw.append_row(row![(),(),()])

}).expect("write excel error!");

wb.write_sheet(&mut edges, |sheet_writer| {
    let sw = sheet_writer;
    sw.append_row(row!["Target", "Source","Type","Label"])?;

    for row in &table {
        let id=row.get("Id").unwrap_or("<id missing>");
        let item=row.get("Item").unwrap_or("<item missing>");
        let azione=row.get("Azione").unwrap_or("<azione missing>");

        let mut next_list:Vec<Next>=get_next(row.get("Next").unwrap_or("<next missing>"));

        for next in next_list.iter() {
            sw.append_row(row![next.link.to_string(),id,"D",next.action.to_string()])?;
        }
       
    }
    sw.append_row(row![(),(),()])
}).expect("write excel error!");



/*
  wb.write_sheet(&mut sheet, |sheet_writer| {
    let sw = sheet_writer;
    sw.append_row(row!["Name", "Title","Success","XML Remark"])?;
    sw.append_row(row!["Amy", (), true,"<xml><tag>\"Hello\" & 'World'</tag></xml>"])
    }).expect("write excel error!");
*/

  wb.close().expect("close excel error!");

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
                if z.len()==1 && z[0].is_empty(){
                    continue;
                } else if z.len()==1 && !z[0].is_empty(){
                    let next=Next {
                        action:String::from(""),
                        link:get_link(&v[0].to_string()),
                };
                next_list.push(next); 
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
