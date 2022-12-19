use std::io::{self, Read, Result};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::id;
use std::str;
use export_drawio::{CellType, Cell};
use substring::Substring;
extern crate table_extract as table;


mod export_drawio;


#[derive(Debug)]
struct Next {
    action: String,
    link:String
}

fn main() -> Result<()>{

    // let mut filename = String::from("./samples/FE706 - SV - Filodiffusione.htm");
    //let mut filename = String::from("./samples/FE251 - FO - Borchia GSM.htm");
    let mut filename = String::from("./samples/AAA_CONNETTIVITA_RIEPILOGO.htm");
    //let mut filename = String::from("./samples/FE110 - Navigazione Lenta NEW.htm");

    let file = File::open(&filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut html = String::new();
    buf_reader.read_to_string(&mut html)?;

    let table = table_extract::Table::find_first(&html).unwrap();
    filename.push_str(".drawio.xml");
    let mut file = File::create(filename)?;
   

   let mut cell_list:Vec<export_drawio::Cell>=Vec::new();

   let mut node_stack:Vec<String>=Vec::new();
   let mut row_stack:Vec<table::Row>=Vec::new();

   // let mut row_stack:Vec<Row>=Vec::new();

   let id="AUT_AMB_ESTAR[8857]_25510";

   node_stack.push(String::from(id));


   loop {
    println!("### Stack size: {}",node_stack.len());
    println!("### Stack items: {:?}",node_stack);
    
    let node=node_stack.pop().unwrap();

    println!("Get node: {}",node);

    let row=table.iter().find(|x| x.get("Id").unwrap()==node);

    match row {
        Some(x) => {
            println!("node: {}",x.get("Id").unwrap_or("<id missing>"));

            let mut i=0;

            for r in &row_stack {
                if r.get("Id").unwrap().eq(node.as_str()) {break;}
                i+=1;
            }
            if i==row_stack.len(){
                row_stack.push(x);
            }

            let next_list:Vec<Next>=get_next_item(x.get("Next").unwrap_or("<next missing>"));

            println!(">>{:?}<<",next_list);

            for next in next_list.iter() {
                match row_stack.iter().find(|x| x.get("Id").unwrap()==next.link) {
                    Some(x) => {continue},
                    _ => {},
                };
                node_stack.push(next.link.to_string());
              }

        },
        _ => {},

    };

    if node_stack.len()==0 {break};

   }
   
   println!("Cell Type: {:?}",get_celltype_from_item("Stella"));

   // push id into stack
   // loop ...
   //   pop from stack
   //   get node from table by id
   //   get children from node
   //   push children to stack
   //   break from loop when stack is empty
   

    for row in &row_stack {
        //if index > 1 {break;}
        
        let id=row.get("Id").unwrap_or("<id missing>");
        let item=row.get("Item").unwrap_or("<item missing>");
        let action=row.get("Azione").unwrap_or("<azione missing>").trim();
       
        if (item=="Stampa") {continue;}

        let next_list:Vec<Next>=get_next_item(row.get("Next").unwrap_or("<next missing>"));

        
        //let mut cell_item_type=CellType::RECTANGLE;

        // create cell node
        //if item=="Scelta" {
        //    cell_item_type=CellType::DIAMON;
        //}
        
        //let cell_item_type_tmp= cell_item_type;

       // let mut escaped_item=xml::escape::escape_str_attribute(item);

        let cell=export_drawio::Cell {id:String::from(id), text:String::from(xml::escape::escape_str_attribute(item)), 
        tooltip:String::from(xml::escape::escape_str_attribute(action)),
        geometry:export_drawio::Cell::get_default_geometry(get_celltype_from_item(item)),
        cell_type:get_celltype_from_item(item),
        source:String::from("0"), target:String::from("0")};
        cell_list.push(cell);

        let mut index=0;

        for next in next_list.iter() {
            
            //if (String::from(&next.action)!="Altrimenti") {continue;}
            //create edges
            let mut edge_id=format!("{}-{}",id,index);
            let mut text=String::from(next.action.as_str());
            let mut tooltip=String::from(next.action.as_str());

            if text.len()>50 {
                text=format!("edge-{}",index);
            }
            let cell=export_drawio::Cell {id:edge_id, text:String::from(xml::escape::escape_str_attribute(&text).trim()), 
                tooltip:String::from(xml::escape::escape_str_attribute(&tooltip).trim()),geometry:export_drawio::Cell::get_default_geometry(CellType::EDGE_WITH_LABEL),cell_type:CellType::EDGE_WITH_LABEL,
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

fn get_celltype_from_item(item:&str) -> export_drawio::CellType{

    match String::from(xml::escape::escape_str_attribute(item)).as_str() {
      "Scelta"  => CellType::TRAPEZOID,
      "Choice"  => CellType::DIAMON,
      "Stella"  => CellType::DOCUMENT,
      "Raggio"  => CellType::PARALLELOGRAM,
      "Jump"    => CellType::STEP,
      "Scelta Utente" => CellType::HEXAGON,
      "Callback"      => CellType::TRIANGLE,
      _ => CellType::RECTANGLE
    }
}

// txt1&nbsp;<a..>link1</a><br>txt2&nbsp;link2
fn get_next_item(td:&str) -> Vec<Next> { 

    let mut next_list:Vec<Next> = Vec::new();

    let mut next:Next = Next{action:String::from(""),link:String::from("")};

    let v: Vec<&str> = td.split("</a>").collect();

    let mut index=0;

    if v.len()>1 {
        for a in v.iter() {
            if index> v.len()-2 {break;}

            let b:Vec<&str>=a.split("<a").collect();
            if b.len()>1 {
                let _link=match get_link(&b[1].to_string()){
                    Some(x) => x,
                    None => continue,
                };
                 next=Next {
                    action:string_clean(b[0].to_string()),
                    link:_link,
               };
            } else {
                let _link=match get_link(&b[0].to_string()){
                    Some(x) => x,
                    None => continue,
                };
                next=Next {
                    action:String::from(""),
                    link:_link,
                };
            }
            next_list.push(next);
            index+=1;
        }
    } else {
        let _link=match get_link(&v[0].to_string()){
            Some(x) => x,
            None => return next_list,
        };
        next=Next {
            action:String::from(""),
            link:_link,
        };
        next_list.push(next);
    }
    
    next_list
}


// txt1&nbsp;<a..>link1</a><br>txt2&nbsp;link2
/*
fn get_next(td:&str) -> Vec<Next>{ 
        // TODO: action is an option
        
        let mut next_list:Vec<Next> = Vec::new();

        let v: Vec<&str> = td.split("<a").collect();

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
 */  

fn string_clean(s:String) -> String{
      s.replace(&['\n','\t',' '][..],"")
 }

 fn get_link(link:&String) -> Option<String>{

   
    if link.len()==0 || link.eq("") || !link.contains("href") {
        return None;
    }

    let start_index= match link.find("\">"){
        Some(x)=> x,
        None => 0,   
        };
/*
        let end_index= match link.find("\">"){
            Some(x)=> x,
            None => link.len(),   
         };
*/
        let _link=link.substring(start_index+2, link.len());
        return Some(string_clean(_link.to_string()));
    }

fn split_td(td: &str) -> Vec<&str>{

        let v: Vec<&str> = td.split("<br>").collect();

        return v;

        //<a href="#AUT_AMB_ESTAR[2837]_139776">AUT_AMB_ESTAR[2837]_139776</a>

    }
