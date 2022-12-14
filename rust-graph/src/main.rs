use std::io::{self, Read, Result};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::str;
use export_drawio::CellType;
use substring::Substring;
extern crate table_extract;


mod export_drawio;


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

    let table = table_extract::Table::find_first(&html).unwrap();
    filename.push_str(".drawio.xml");
    let mut file = File::create(filename)?;
   

    let mut cell_list:Vec<export_drawio::Cell>=Vec::new();
   
    for row in &table {
        //if index > 1 {break;}
        let id=row.get("Id").unwrap_or("<id missing>");
        let item=row.get("Item").unwrap_or("<item missing>");
        let action=row.get("Azione").unwrap_or("<azione missing>");
        let mut next_list:Vec<Next>=get_next(row.get("Next").unwrap_or("<next missing>"));

        // create cell node
        let cell=export_drawio::Cell {id:String::from(id), text:String::from(item), 
        tooltip:String::from(xml::escape::escape_str_attribute(action)),geometry:export_drawio::Cell::get_default_geometry(CellType::RECTANGLE),cell_type:CellType::RECTANGLE,
        source:String::from("0"), target:String::from("0")};
        cell_list.push(cell);

        let mut index=0;

        for next in next_list.iter() {
            //create edges
            let mut edge_id=format!("{}-{}",id,index);
            let cell=export_drawio::Cell {id:edge_id, text:String::from(&next.action), 
                tooltip:String::from(action),geometry:export_drawio::Cell::get_default_geometry(CellType::EDGE_WITH_LABEL),cell_type:CellType::EDGE_WITH_LABEL,
                source:String::from(id), target:String::from(&next.link)};
            cell_list.push(cell);
            index=index+1;
        }
       
}

let diagram=export_drawio::Diagram{id:String::from("FirstDiagram"),page_name:String::from("FirstPage"),
page_height:780,page_width:1200, cells: cell_list};

let mut dia_list:Vec<export_drawio::Diagram>=Vec::new();

    dia_list.push(diagram);

    let doc= export_drawio::Document{host:String::from("app.diagrams.net"), repo_type:export_drawio::RepositoryType::DEVICE,modified:String::from("2022-12-08T22:27:57.684Z"),
           diagram_list:dia_list };
    
   // println!(">>> {}",export_drawio::Document::to_xml(doc));

    file.write_all(export_drawio::Document::to_xml(doc).as_bytes())?;

 /*
    for row in &table {
        let id=row.get("Id").unwrap_or("<id missing>");
        let item=row.get("Item").unwrap_or("<item missing>");
        let action=row.get("Azione").unwrap_or("<azione missing>");

        let mut next_list:Vec<Next>=get_next(row.get("Next").unwrap_or("<next missing>"));

        for next in next_list.iter() {
            //create edges
            let cell=export_drawio::Cell {id:String::from(id), text:String::from(item), 
                tooltip:String::from(action),geometry:export_drawio::Cell::get_default_geometry(CellType::RECTANGLE),cell_type:CellType::RECTANGLE,
                source:String::from("0"), target:String::from("0")};
            sw.append_row(row![next.link.to_string(),id,"D",next.action.to_string()])?;
        }
   
    } */


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
