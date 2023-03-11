use std::io::{self, Read};
use std::collections::{HashMap, BTreeMap};
use anyhow::Ok;
use anyhow::{anyhow, bail, Context, Result};
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::process::id;
use std::str;
use export_drawio::{CellType, Cell};
use rand::seq::index;
use simple_excel_writer::Row;
use substring::Substring;
extern crate table_extract as table;
use std::env;

mod export_drawio;

macro_rules! dbg {
    ($x:expr) => {
        println!("{} = {:?}",stringify!($x),$x);
    }
}

#[derive(Debug)]
struct Next {
    action: String,
    link:String
}

#[derive(Debug)]
pub enum LogLevel {
    INFO,
    DEBUG,
    ERROR
}

#[derive(Debug)]
struct CommonInfo{
    root: String,
    current_root: String,
    verbosity: LogLevel
}

///
/// visited_node_stack -> stack containing the nodes visited during the tree path
/// tree_stack -> stack containing all the nodes and edges of the tree
/// shrunk_tree_stack -> compressed tree where linked nodes with same type are compressed
/// 
/// node struct -> id, label, tooltip, outgoing_edge_list, fan_in, fan_out
/// 
/// shrinking rule:
/// 1) nodes must be of the same type
/// 2) nodes must be linked
/// 3) current node must have a fan_out = 1
/// 4) next node must have a fan_in=1
/// 

#[derive(Debug, Clone)]
struct Edge {
    id: String,
    label: String,
    tooltip: String,
    source: String,
    target: String
}

#[derive(Debug)]
struct Node {
    id: String,
    label: String,
    tooltip: String,
    outgoing_edge_list: Vec<Edge>,
    fan_in: u32,
    fan_out: u32,
    is_visited: bool,
    is_merged: bool
} 

impl CommonInfo {
    fn is_debug(info:&mut CommonInfo) -> bool {
        match info.verbosity {
            LogLevel::DEBUG => return true,
            _ => return false,
        }
    }
    fn is_info(info:&mut CommonInfo) -> bool {
        match info.verbosity {
            LogLevel::INFO => return true,
            _ => return false,
        }
    }
    fn is_error(info:&mut CommonInfo) -> bool {
        match info.verbosity {
            LogLevel::ERROR => return true,
            _ => return false,
        }
    }
}

fn main() -> Result<()>{
    let args: Vec<String> = env::args().collect();
   
    if args.len() != 2 {
        eprintln!("Usage: gen-tree filename");
        eprintln!();
        eprintln!("Filename is the path to html table file");
        bail!("Aborted");
    }

    let filename = String::from(&args[1]);
    let file = File::open(&filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut html = String::new();
    buf_reader.read_to_string(&mut html)?;
    

    println!("**********************************************************************");
    println!("gen-tree - Generate draw.io graph from html table tree structure");
    println!("by antvale ");
    println!("**********************************************************************");
    println!();
    println!("Opening \"{}\" read-only.", args[1]);
    let table = table_extract::Table::find_first(&html).unwrap();
    let roots:Vec<String>=retrieve_roots_from_table(&table);
    println!("Extracted \"{}\" nodes with {} roots", table.iter().count(),roots.len());
    println!();

    //let mut current_root=String::from("");
    let mut common_info=CommonInfo{root:String::from(""),current_root:String::from(""),verbosity:LogLevel::INFO};
    let mut tree=Vec::new();

    // extract the root of the tree
    let mut root=String::from("\\");
    /*
    for row in &table
     {
        root.push_str(row.get("Id").unwrap_or("<id missing>"));
        break;
    }
    */

    common_info.root.push_str(&root);
    common_info.current_root.push_str(&root);
    
    //build the tree from the current root
    build(&table, &mut common_info,&mut tree);

    loop {
        print!("tree-shell:\\{}> ",common_info.current_root);
        io::stdout().flush()?;
        let mut input_string = String::new();
        io::stdin().read_line(&mut input_string).unwrap();
        if input_string.is_empty() {
            // An empty `input_string` without even a newline looks like STDIN was closed.
            break;
        }
        let input = input_string.trim();
        let mid = input.find(' ').unwrap_or(input.len());
        let (command, arg) = input.split_at(mid);
        let arg = arg.trim_start();
        
        let result = match command {
        "cd"            => cd(arg, &table,&mut tree,&mut common_info),
        "compact"       => compact(arg, &mut tree, &mut common_info),
        "export"        => export(arg,&mut tree,&mut common_info,&filename),
        "info"          => info(arg,&mut tree),
        "debug"         => debug(arg,&mut common_info),
        "list"          => list(arg,&mut tree, &roots),
        "help"          => help(arg),
        "test"          => test(arg,&mut tree),
        "exit"|"quit"   => break,
        ""              => continue,
        _               => Err(anyhow!(
                "Invalid command \"{}\". Type \"help\" to get a list of all commands.",
                command
            )),
        };
        if let Err(e) = result {
            eprintln!("Error: {:?}", e);
        }
    }

    Ok(())
    }

fn test(arg: &str, tree: &mut Vec<Node>) -> Result<()> {
    //let mut tooltips:Vec<String>=
    let s=tree.iter()
    .filter(|x|x.label.eq_ignore_ascii_case("Imposta Meta Diagnosi"))
    .take(5).map(|x|x.tooltip.to_string())
    .fold(String::from("Diagnosi:"), |acc,x| format!("{}{}",acc,x));
    println!("{}",s);
    //.collect::<Vec<String>>();
    //println!("{:?}",tooltips);
    Ok(())
}
fn help(arg: &str) -> Result<()> {
    match arg {
        "info"      => {
            println!("Usage: info options");
            println!();
            println!("Shows information about the selected tree or a specific node");
        }
        "compact"   => {
            println!("Usage: compact");
            println!();
            println!("Reduce the tree size compacting the items where possible");
        }
        "export"    => {
            println!("Usage: export");
            println!();
            println!("Export the tree in drawio graph format");
        }
        "cd"    => {
            println!("Usage: cd parent");
            println!();
            println!("Change the parent of the tree and build it");
        }
        "get"    => {
            println!("Usage: get options");
            println!();
            println!("Retrieve one or more node based on the options");
        }
        "debug"    => {
            println!("Usage: debug on | off");
            println!();
            println!("On/Off the debug logging");
        }
        "list"    => {
            println!("Usage: list options");
            println!();
            println!("List tree element based on options: impostazioni|domande|decisioni|jump|diagnosi|stelle|opzioni|servizi|riscontri|cic|");
        }
        _ => {
            println!("Available Commands:");
            println!("  compact     - Reduce the size of the tree");
            println!("  export      - Export the tree as drawio file");
            println!("  cd          - Change the root and build the tree");
            println!("  info        - Show info about the selected tree");
            println!("  list        - List the node starting from a parent");
            println!("  get         - Retrieve specific nodes based on options");
            println!("  debug       - Switch log to debug");
            println!("  help        - Show this help");
            println!("  quit        - Quit gen-tree");
            println!();
            println!(
                "You can also enter \"help COMMAND\" to get additional help about some commands."
            );

        }
    }
    Ok(())
}

fn debug(arg:&str,info: &mut CommonInfo) -> Result<()>{
    if arg.is_empty() {
        return Ok(());
    }
    match arg {
        "on"    => info.verbosity=LogLevel::DEBUG,
        "off"   => info.verbosity=LogLevel::INFO,
        _       => info.verbosity=LogLevel::ERROR
    }
    Ok(())
}

fn cd (arg:&str, table:&table_extract::Table, tree: &mut Vec<Node>, info:&mut CommonInfo)-> Result<()>{

    if arg.is_empty() {
        info.current_root=String::from(&info.root);
    } else {
        info.current_root=String::from(arg);
    }
    tree.clear();
    build(table, info, tree);

    Ok(())
}

/// Build the tree from the reverse html table and starting from the current root
fn build (table:&table_extract::Table,info:&mut CommonInfo,tree: &mut Vec<Node>) -> Result<()>{
    
    let mut visited_node_stack:Vec<String>=Vec::new();
    let mut edge_visited_stack:Vec<(String,String)>=Vec::new();
    //let mut tree_stack:Vec<Node>=Vec::new();

    if info.current_root.eq("\\") {
        //build all the forest
        for row in table {
            tree.push(convert_row2node(false, row));
        }
        return Ok(());
    }

    visited_node_stack.push(String::from(&info.current_root));

    loop {

        if CommonInfo::is_debug(info){
            println!("### Stack size: {}",visited_node_stack.len());
        }
       
        let node_id=visited_node_stack.pop().unwrap();
        
        if CommonInfo::is_debug(info){
            println!("Got node with id: {}",node_id);
        }
        //retrieve the row from html table by id
        let row=table.iter().find(|x| x.get("Id").unwrap()==node_id);
    
        match row {
            Some(x) => {
                if CommonInfo::is_debug(info){
                    println!("[Current node: {}]",x.get("Id").unwrap_or("<id missing>"));
                }
                let mut i=1;
                
                for n in tree.iter() {
                    if n.id.eq(node_id.as_str()) {break;}
                    i+=1;
                }
                
                if i==tree.len()+1 {
                    //convert the row to node and push in the tree stack
                    tree.push(convert_row2node(true,x));
                    if CommonInfo::is_debug(info){
                        println!("[Current node: {}] is a new node -> pushed to tree stack",&node_id);
                    }
                }
                
    
                let mut node=tree.get_mut(i-1).unwrap();

                node.fan_in+=1;
                if CommonInfo::is_debug(info){
                    println!("[Current node: {}] Fan-in increased -> new value: {}",&node_id,&node.fan_in);
                }
    
                for e in &node.outgoing_edge_list{
                    
                    edge_visited_stack.iter().find(|(x,y)|*x==node.id && *y==e.target);
                    if edge_visited_stack.contains(&(String::from(&node.id),String::from(&e.target))) {
                        if CommonInfo::is_debug(info){
                            println!("[Current node: {}] Already traversed here ({},{})",&node_id,&node_id,e.target);
                        }
                        continue;
                    } else {
                        edge_visited_stack.push((String::from(&node.id),String::from(&e.target)));
                        visited_node_stack.push(String::from(&e.target));
                    }
                }
                if CommonInfo::is_debug(info){
                    println!("[Current node: {}] Added new outgoing edges to visiting node stack -> length:{}",&node_id,&node.fan_out);
                }
    
    
            },
            _ => {},
    
        };
    
        if visited_node_stack.len()==0 {break};
    
       }

       Ok(())
}

/*
fn find_or_insert_mut<'a, Node:PartialEq>(vec: &mut Vec<Node>, val: Node, id:&str) -> &mut Node{
    if let Some(i)=vec.iter().position(|x| *x.id == id){
        &mut vec[i]
    }else {
        vec.push(val);
        vec.last_mut().unwrap()
    }
                
}
*/

// reduce the size of the tree merging similar items
fn compact(arg: &str,tree:&mut Vec<Node>, info: &mut CommonInfo) -> Result<()>{

    let mut merged_counter=0;
    let mut current_position=0;

    loop {
 
        let current_node=tree.get(current_position).unwrap();
        if CommonInfo::is_debug(info){
            println!("### Get current node at position: {} => {}",current_position,current_node.id);
        }
        if current_node.label.eq("Scelta Utente") && current_node.fan_out==1{
    
            let edge_id=&current_node.outgoing_edge_list.get(0).unwrap().target;
            if CommonInfo::is_debug(info){
                println!("### Current Node Id: {}",current_node.id);
                println!("### Current Node Type: {}",current_node.label);
                println!("### Current Node FanOut: {}",current_node.fan_out);
                println!("### Current Node: edge list length {}",current_node.outgoing_edge_list.len());
            }
            //get next node
            //let result_next_node=tree_stack.iter_mut().find(|x| x.id.eq(edge_id.as_str()));
            let mut next_position=0;
            for r in tree.iter() {
                if r.id==*edge_id {break;}
                next_position+=1;
            }
            if (next_position==tree.len()){
                if CommonInfo::is_debug(info){
                    println!("### Current Node: No adjacent node found!");
                }
                continue;
            }
    
            let next_node=tree.get(next_position).unwrap();
            if CommonInfo::is_debug(info){
                println!("### Current Node: Got adjacent node: {}",next_node.id);
            }
    
            if next_node.label.eq("Scelta Utente") && next_node.fan_in==1 {
            
    
        // compact two nodes
    
        let mut new_outgoing_edge_list:Vec<Edge>=Vec::new();
    
        for e in &next_node.outgoing_edge_list{
    
            let edge=Edge{
                id:String::from(&e.id),
                label:String::from(&e.label),
                tooltip:String::from(&e.tooltip),
                source:String::from(&current_node.id),
                target:String::from(&e.target)
            };
            new_outgoing_edge_list.push(edge);
        }

        let new_node= Node {
            id:String::from(&current_node.id),
            label: format!("{}",current_node.label),
            tooltip: format!("{}{}",current_node.tooltip,next_node.tooltip),
            outgoing_edge_list:new_outgoing_edge_list,
            fan_in:current_node.fan_in,
            fan_out:next_node.fan_out,
            is_visited:false,
            is_merged:false
        };
        if CommonInfo::is_debug(info){
            println!("### Current Node: Merge two node current {} and next{} and push to tree_stack back",current_node.id,next_node.id);
        }
        tree.remove(current_position);
        if (next_position> current_position) {next_position-=1;} else {current_position-=1;}
        tree.remove(next_position);
        
    
        merged_counter+=1;
        
        //current_node.is_merged=true;
        //next_node.is_merged=true;
        if CommonInfo::is_debug(info){
            println!("Inserting {} at position {}",new_node.id,tree.len() );
        }
        tree.insert(tree.len(),new_node);
    
        current_position-=1;
        
        //continue;
        }
        }
        current_position+=1;
    
        if tree.len()==current_position {break;}

    }
   Ok(())
}


// export the tree in drawio format
fn export(arg: &str,tree:&Vec<Node>,info:&mut CommonInfo,output_filename:&str) -> Result<()> {
    
    let mut filename=String::from(output_filename);
    filename.push_str(format!("-{}.drawio.xml",info.current_root).as_str());

    let mut file = File::create(&filename)?;
   

    let mut cell_list:Vec<export_drawio::Cell>=Vec::new();


 /*
   println!("Item number after compression: {}",tree_stack.len());
   println!("#Scelta Utente: {}",tree_stack.iter().filter(|x| x.label=="Scelta Utente").count());
   println!("Counters => Merged nodes:{}",merged_counter*2);
   
   let item_num_f64=f64::from(i32::try_from(tree_stack.len()).unwrap());
   let initial_item_num_f64=f64::from(i32::try_from(initial_item_num).unwrap());
   let compression_rate=(1_f64-item_num_f64/initial_item_num_f64)*100_f64;

   println!("Compression rate: {:.1}%", compression_rate);
   */
   
   for n in tree{

    let cell=export_drawio::Cell {id:String::from(&n.id), text:String::from(xml::escape::escape_str_attribute(&n.label)), 
        tooltip:String::from(xml::escape::escape_str_attribute(&n.tooltip)),
        geometry:export_drawio::Cell::get_default_geometry(get_celltype_from_item(&n.label)),
        cell_type:get_celltype_from_item(&n.label),
        source:String::from("0"), target:String::from("0")};
    cell_list.push(cell);

    let mut index=0;

    for e in &n.outgoing_edge_list{
        //visited_node_stack.push(String::from(&e.target));

        //let mut edge_id=format!("{}-{}",&n.id,index);
        let mut text=String::from(e.label.as_str());
        let mut tooltip=String::from(e.label.as_str());

            if text.len()>50 {
                //text=text.truncate(50);
                text.truncate(50);
                text=format!("{}...",&text);
            }
            let cell=export_drawio::Cell {id:String::from(&e.id), 
                text:String::from(xml::escape::escape_str_attribute(&text)), 
                tooltip:String::from(xml::escape::escape_str_attribute(&e.tooltip)),
                geometry:export_drawio::Cell::get_default_geometry(CellType::EDGE_WITH_LABEL),
                cell_type:CellType::EDGE_WITH_LABEL,
                source:String::from(&e.source), 
                target:String::from(&e.target)
            };
            
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
    
        file.write_all(export_drawio::Document::to_xml(doc).as_bytes())?;
       
        println!("Written output file: {}",&filename);
    
    Ok(())
}

fn list(arg:&str, tree:&mut Vec<Node>, roots:& Vec<String>) -> Result <()>{

    match arg {
        "domande" => tree.iter().filter(|x| x.label.eq("Scelta")).for_each(|x| println!("{}",x.id)),
        "stelle" => tree.iter().filter(|x| x.label.eq("Stella")).for_each(|x| println!("{}",x.id)),
        "jump" => iter_filter_count_print(tree,"Jump"),
        "servizi" => iter_filter_count_print(tree,"Servizio Esterno"),
        "decisioni" => tree.iter().filter(|x| x.label.eq("Choice")).for_each(|x| println!("{}",x.id)),
        "opzioni" => tree.iter().filter(|x| x.label.eq("Raggio")).for_each(|x| println!("{}",x.id)),
        "impostazioni" => tree.iter().filter(|x| x.label.eq("Scelta Utente")).for_each(|x| println!("{}",x.id)),
        "radici" => roots.iter().for_each(|x| println!("{}",x)),
        "diagnosi" => iter_filter_count_print(tree,"Imposta Meta Diagnosi"),
        "riscontri" => iter_filter_count_print(tree,"Imposta Meta Riscontro"),
        "cic" => iter_filter_count_print(tree,"Verifica CIC"),
        _  => tree.iter().for_each(|x| println!("{}",x.id)),
    }
    Ok(())
}

fn iter_filter_count_print(tree:&mut Vec<Node>, filter:&str) {
    let mut tooltips:Vec<String>=tree.iter().filter(|x| x.label.eq_ignore_ascii_case(filter)).
    map(|x|x.tooltip.to_string()).collect::<Vec<String>>();
    tooltips.sort();
    tooltips.dedup();
    tooltips.iter().for_each(|x| println!("{}",x));
    println!("{:34}{}",format!("N.ro {}",filter),tooltips.len());
}

fn info(arg: &str,tree:&mut Vec<Node>) -> Result<()>{
    
    if arg.is_empty(){
        println!("{:34}{}","N.ro Impostazioni",tree.iter().filter(|x| x.label=="Scelta Utente").count());
        println!("{:34}{}","N.ro Domande",tree.iter().filter(|x| x.label=="Scelta").count());
        println!("{:34}{}","N.ro Decisioni",tree.iter().filter(|x| x.label=="Choice").count());
        println!("{:34}{}","N.ro Jump",tree.iter().filter(|x| x.label=="Jump").count());
        println!("{:34}{}","N.ro Stelle",tree.iter().filter(|x| x.label=="Stella").count());
        println!("{:34}{}","N.ro Diagnosi",tree.iter().filter(|x| x.label=="Imposta Meta Diagnosi").count());
        println!("{:34}{}","N.ro Servizi",tree.iter().filter(|x| x.label=="Servizio Esterno").count());
        println!("{:34}{}","N.ro Riscontri",tree.iter().filter(|x| x.label=="Imposta Meta Riscontro").count());
        println!("{:34}{}","N.ro Cic",tree.iter().filter(|x| x.label=="Verifica CIC").count());
    } else {
        match arg {
            "diagnosi" => tree.iter().filter(|x| x.label.eq("Imposta Meta Diagnosi")).for_each(|x| println!("{}",x.tooltip)),
            /*"riscontri" => tree.iter().filter(|x| x.label.eq("Imposta Meta Riscontro")). 
            .collect::<Vec<&Node>>()
           
            .sort_by(|a,b| a.tooltip.cmp(&b.tooltip)). .for_each(|x| println!("{}",x.tooltip)),*/
            _  => tree.iter().filter(|x| x.id.eq(arg)).for_each(|x| println!("{:?}",x)),
        }
    } 
    //tree.iter().collect().sort_by(|a,b| a.id.cmp(&b.id));
    Ok(())
}
pub fn copy_vec<T: Clone>(vec: &Vec<T>) -> Vec<T> {
    let mut vec = vec.clone();
    vec
}

fn convert_row2node(debug:bool,row:table::Row)-> Node {

    let id=row.get("Id").unwrap_or("<id missing>");
    let item=row.get("Item").unwrap_or("<item missing>");
    let action=row.get("Azione").unwrap_or("<azione missing>").trim();

    let mut outgoing_edge_list:Vec<Edge>=Vec::new();
    
    let next_list:Vec<Next>=get_next_item(row.get("Next").unwrap_or("<next missing>"));

    let mut index=0;

    for next in next_list.iter() {
        
        let edge_id=format!("{}-{}",id,index);
        let text=String::from(next.action.as_str());
        let tooltip=String::from(next.action.as_str());

        let edge=Edge {
            id:edge_id,
            label:string_clean_br(text),
            tooltip:string_clean_br(tooltip),
            source:String::from(id), 
            target:String::from(&next.link)
        };
        outgoing_edge_list.push(edge);    
        index=index+1;
           
        }
    
    let mut node_tooltip= action.to_string();

    if debug {
        node_tooltip=format!("{}\n{}",id,action.to_string())
    }

    Node { id: id.to_string(), 
           label: item.to_string(), 
           tooltip: node_tooltip, 
           outgoing_edge_list: outgoing_edge_list, 
           fan_in: 0, 
           fan_out: index,
           is_visited:false,
           is_merged:false
        }

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

fn retrieve_roots_from_table(table:&table::Table) -> Vec<String>{

    let mut edge_list:Vec<String>=Vec::new();
    let mut node_list:Vec<String>=Vec::new();

    let mut edge_map:BTreeMap<String, i32>=BTreeMap::new();

    let mut index=1;

    for row in table {
        let id=row.get("Id").unwrap_or("<next missing>");
      
        node_list.push(id.to_string());

        let next_list:Vec<Next>=get_next_item(row.get("Next").unwrap_or("<next missing>"));

        for next in &next_list{
            if let Some(value)= edge_map.get_mut(&next.link.to_string()){
                *value+=1;
            } else{
                edge_map.insert(next.link.to_string(), 1);
            }
            
        }
      
        index+=1;

    }
    
    let mut root_list:Vec<String>=Vec::new();

    for n in &node_list{
        if edge_map.contains_key(n) {continue;}
        root_list.push(n.to_string());
    }

    root_list
    
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


fn string_clean(s:String) -> String{
      s.replace(&['\n','\t'][..],"");
      s.as_str().replace("<br>", " ").trim().to_string()
 }

 fn string_clean_br(s:String) -> String{
    string_clean(s).as_str().replace("<br>", " ").to_string()
 }

 fn get_link(link:&String) -> Option<String>{

    if link.len()==0 || link.eq("") || !link.contains("href") {
        return None;
    }

    let _link=string_clean(link.to_string()).trim().to_string();

    let mut start_index=0;
    let mut end_index=_link.len();

    if link.contains("Al ritorno"){
        start_index= match _link.find("href=\"#"){
            Some(x)=> x+7,
            None => 0,   
            };
        end_index= match _link.find("\">"){
            Some(x)=> x,
            None => 0,   
            };
        } else {
            start_index= match _link.find("\">"){
                Some(x)=> x+2,
                None => 0,   
                };
            end_index=_link.len();
        }

        let __link=_link.substring(start_index, end_index);
        return Some(__link.to_string());
    }

fn split_td(td: &str) -> Vec<&str>{

        let v: Vec<&str> = td.split("<br>").collect();

        return v;

        //<a href="#AUT_AMB_ESTAR[2837]_139776">AUT_AMB_ESTAR[2837]_139776</a>

    }
