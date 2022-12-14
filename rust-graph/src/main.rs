use std::{io::{self, Read, Result}, fmt::format};
use rand::Rng;

macro_rules! dbg {
    ($x:expr) => {
        println!("{} = {:?}",stringify!($x),$x);
    }
}
trait PrintShape{
    fn print(&self);
}

#[derive(Debug)]
enum RepositoryType {
    DEVICE,
    GOOGLE_DRIVE,
    ONEDRIVE
}

#[derive(Debug)]
struct Diagram {
    id:String,
    page_name:String,
    page_width:u16,
    page_height:u16,
    cells: Vec<Cell>
}


#[derive(Debug)]
struct Document {
    host: String,
    repo_type: RepositoryType,
    modified: String,
    diagram_list: Vec<Diagram>
 
}


#[derive(Debug)]
struct Geometry {
    relative: u8,
    width: u16,
    height: u16,
    x: u16,
    y: u16
}
#[derive(Debug)]
struct Node{
    id: String,
    style: String,
    text: String,
    tooltip: String,
    geometry: Geometry
}
#[derive(Debug)]
struct Edge{
    id: String,
    style: String,
    text: String,
    target: String,
    source: String
}

#[derive(Debug)]
enum CellType {
    EDGE,
    EDGE_WITH_LABEL,
    RECTANGLE,
    DIAMON,
    DOCUMENT
}

#[derive(Debug)]
struct Cell{
    id: String,
    cell_type: CellType, 
    text: String,
    tooltip: String,
    geometry: Geometry,
    target: String,
    source: String
}
impl Diagram {
    fn to_xml(diagram:Diagram) -> String{
        
        let mut d=format!("<diagram id=\"{id}\" name=\"{page}\">
        <mxGraphModel dx=\"2013\" dy=\"1883\" grid=\"1\" gridSize=\"10\" guides=\"1\" tooltips=\"1\" connect=\"1\" arrows=\"1\" fold=\"1\" page=\"1\" pageScale=\"1\" pageWidth=\"{page_width}\" pageHeight=\"{page_height}\" math=\"0\" shadow=\"0\">
          <root> <mxCell id=\"0\" />
          <mxCell id=\"1\" parent=\"0\" />",id=diagram.id,page=diagram.page_name,page_width=diagram.page_width,page_height=diagram.page_height);
          for c in diagram.cells {
            d.push_str(Cell::to_xml(&c).as_str());
          }
          d.push_str("</root>
          </mxGraphModel>
        </diagram>");
        return d;
    }
   
}


impl Document {
    fn to_xml (doc: Document) -> String {
        let mut d=format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>
<mxfile host=\"{host}\" modified=\"{modified}\" agent=\"5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/14.1.2 Safari/605.1.15\" etag=\"O5BAPYlSG-wtOZ_Z30hE\" version=\"20.6.1\" type=\"device\">",
    host=doc.host, modified=doc.modified);

    for diagram in doc.diagram_list{
        d.push_str(Diagram::to_xml(diagram).as_str());
    }

    d.push_str("</mxfile>");
    return d;

    }
}

impl Cell {
    fn get_default_geometry(cell_type:CellType) -> Geometry {

        match cell_type {
            CellType::RECTANGLE => 
            return Geometry{
                width:120,
                height:60,
                x:0,
                y:0,
                relative:0
            },
            CellType::DIAMON => 
            return Geometry{
                width:60,
                height:60,
                x:0,
                y:0,
                relative:0
            },
            CellType::EDGE =>
            return Geometry{
                width:0,
                height:0,
                x:0,
                y:0,
                relative:1
            },
            CellType::EDGE_WITH_LABEL =>
            return Geometry{
                width:0,
                height:0,
                x:0,
                y:0,
                relative:1
            },
            CellType::DOCUMENT =>
            return Geometry{
                width:120,
                height:60,
                x:0,
                y:0,
                relative:0
            },
    }
}

    fn to_xml (cell: &Cell) -> String {
        let mut rng =rand::thread_rng();

        match cell.cell_type {
            CellType::RECTANGLE =>
                format!("<UserObject label=\"{text}\" tooltip=\"{tooltip}\" id=\"{id}\">
<mxCell style=\"rounded=0;whiteSpace=wrap;html=1;sketch=0;shadow=0;fillColor=#dae8fc;strokeColor=#6c8ebf;\" parent=\"1\" vertex=\"1\">
    <mxGeometry x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" as=\"geometry\" />
</mxCell>
</UserObject>",text=cell.text,tooltip=cell.tooltip,id=cell.id,
               x=cell.geometry.x,y=cell.geometry.y,width=cell.geometry.width,height=cell.geometry.height),
            CellType::DIAMON => 
                format!("<UserObject label=\"{text}\" tooltip=\"{tooltip}\" id=\"{id}\">
<mxCell style=\"rhombus;whiteSpace=wrap;html=1;shadow=0;sketch=0;\" vertex=\"1\" parent=\"1\">
    <mxGeometry x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" as=\"geometry\" />
</mxCell>
</UserObject>",id=cell.id, text=cell.text,tooltip=cell.tooltip,x=cell.geometry.x,y=cell.geometry.y,
                         width=cell.geometry.width, height=cell.geometry.height),
            CellType::EDGE => 
                format!("<mxCell id=\"{id}\" style=\"edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;\" edge=\"1\" parent=\"1\" source=\"{source}\" target=\"{target}\">
    <mxGeometry relative=\"1\" as=\"geometry\" />
</mxCell>",id=cell.id, source=cell.source,target=cell.target),
            CellType::EDGE_WITH_LABEL => 
                format!("<mxCell id=\"{id}\" style=\"edgeStyle=orthogonalEdgeStyle;rounded=0;orthogonalLoop=1;jettySize=auto;html=1;\" edge=\"1\" parent=\"1\" source=\"{source}\" target=\"{target}\">
    <mxGeometry relative=\"1\" as=\"geometry\" />
</mxCell>
<mxCell id=\"{sub_id}\" value=\"{text}\" style=\"edgeLabel;html=1;align=center;verticalAlign=middle;resizable=0;points=[];\" vertex=\"1\" connectable=\"0\" parent=\"{id}\">
    <mxGeometry relative=\"1\" as=\"geometry\">
        <mxPoint as=\"offset\" />
    </mxGeometry>
</mxCell>",sub_id=rng.gen::<u32>(),id=cell.id, text=cell.text,source=cell.source,target=cell.target),
            CellType::DOCUMENT =>
                format!("<UserObject label=\"{text}\" tooltip=\"{tooltip}\" id=\"{id}\">
    <mxCell id=\"{id}\" value=\"{text}\" style=\"shape=document;whiteSpace=wrap;html=1;boundedLbl=1;shadow=0;sketch=0;\" vertex=\"1\" parent=\"1\">
        <mxGeometry x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" as=\"geometry\" />
    </mxCell>
</UserObject>",id=cell.id,text=cell.text,tooltip=cell.tooltip,x=cell.geometry.x,y=cell.geometry.y,width=cell.geometry.width,height=cell.geometry.height),
            _=> format!("isn't a xml"),
        }
        
    }
    
}


impl PrintShape for Node {
    fn print(&self) {
        println!("Print node: {:?}",self);
    }
}

impl PrintShape for Edge {
    fn print(&self) {
        println!("Print edge: {:?}",self);
    }
}

fn printShape<T>(t:T)
    where T:PrintShape
{
    t.print()
}

/*
fn getShape(String: shape_type) -> Shape {

}
*/

trait Show {
    fn show(&self) -> String;
}

trait Location {
    fn location(&self) -> String;
}

trait ShowTell: Show + Location {}

#[derive(Debug)]
struct Foo {
    name: String,
    location: String
}

impl Foo {
    fn new(name: &str, location: &str) -> Foo {
        Foo{
            name: name.to_string(),
            location: location.to_string()
        }
    }
}

impl Show for Foo {
    fn show(&self) -> String {
        self.name.clone()
    }
}

impl Location for Foo {
    fn location(&self) -> String {
        self.location.clone()
    }
}

impl ShowTell for Foo {}

fn main(){

    let g1= Geometry{x:0,y:0,width:120,height:60,relative:0};
    let g2= Geometry{x:120,y:60,width:240,height:120,relative:0};
    let node1 = Node{id:String::from("pluto-1"),style:String::from("anystyle"),
                      text:String::from("I'm node 1"), tooltip:String::from("I'm node 1"), geometry:g1};
    let node2 = Node{id:String::from("pluto-2"),style:String::from("anystyle"),
                      text:String::from("I'm node 2"), tooltip:String::from("I'm node 2"), geometry:g2};
                     
    let edge = Edge{id:String::from("pippo-1"),style:String::from("anystyle"),
    text:String::from("I'm edge from 1 to 2"),
    source:String::from("pluto-1"), target:String::from("pluto-2")};

    printShape(node1);
    printShape(node2);
    printShape(edge);
    let foo = Foo::new("Pete","bathroom");

    dbg!(foo.show());
    /*
    let g3= Geometry{x:120,y:60,width:240,height:120,relative:0};
    let g4= Geometry{x:240,y:280,width:240,height:120,relative:0};
    let g5= Geometry{x:120,y:450,width:60,height:60,relative:0};
    let g6= Geometry{x:120,y:450,width:60,height:60,relative:0};
    let g7= Geometry{x:120,y:450,width:60,height:60,relative:0};
    let g8= Geometry{x:0,y:520,width:60,height:60,relative:0};
    let g9= Geometry{x:100,y:520,width:60,height:60,relative:0};
    let g10= Geometry{x:0,y:520,width:60,height:60,relative:0};
    let g11= Geometry{x:100,y:520,width:60,height:60,relative:0};
     */
    let g3=Cell::get_default_geometry(CellType::RECTANGLE);
    let g4=Cell::get_default_geometry(CellType::RECTANGLE);
    let g5=Cell::get_default_geometry(CellType::EDGE_WITH_LABEL);
    let g6=Cell::get_default_geometry(CellType::DIAMON);
    let g7=Cell::get_default_geometry(CellType::EDGE_WITH_LABEL);
    let g8=Cell::get_default_geometry(CellType::RECTANGLE);
    let g9 = Cell::get_default_geometry(CellType::RECTANGLE);
    let g10=Cell::get_default_geometry(CellType::EDGE_WITH_LABEL);
    let g11=Cell::get_default_geometry(CellType::EDGE_WITH_LABEL);


    let cell1=Cell {id:String::from("minnie-1"), text:String::from("I'm a rectangle"), 
                          tooltip:String::from("I'm a rectangle"),geometry:g3,cell_type:CellType::RECTANGLE,
                          source:String::from("0"), target:String::from("0")};
    let cell2=Cell {id:String::from("minnie-2"), text:String::from("I'm a rectangle"), 
                          tooltip:String::from("I'm a rectangle"),geometry:g4,cell_type:CellType::RECTANGLE,
                          source:String::from("0"), target:String::from("0")};
    let cell3=Cell {id:String::from("minnie-3"), text:String::from("I'm an edge"), 
                          tooltip:String::from("I'm Minnie"),geometry:g5,cell_type:CellType::EDGE_WITH_LABEL,
                          source:String::from("minnie-1"), target:String::from("minnie-2")};
    let cell4=Cell {id:String::from("minnie-4"), text:String::from("I'm an diamon"), 
                          tooltip:String::from("I'm Minnie"),geometry:g6,cell_type:CellType::DIAMON,
                          source:String::from("0"), target:String::from("0")};
    let cell5=Cell {id:String::from("minnie-5"), text:String::from("I'm an edge"), 
                          tooltip:String::from("I'm Minnie"),geometry:g7,cell_type:CellType::EDGE_WITH_LABEL,
                          source:String::from("minnie-2"), target:String::from("minnie-4")};
    let cell6=Cell {id:String::from("minnie-6"), text:String::from("I'm an rectangle"), 
                          tooltip:String::from("I'm Minnie"),geometry:g8,cell_type:CellType::RECTANGLE,
                          source:String::from("0"), target:String::from("0")};
    let cell7=Cell {id:String::from("minnie-7"), text:String::from("I'm an rectangle"), 
                          tooltip:String::from("I'm Minnie"),geometry:g9,cell_type:CellType::RECTANGLE,
                          source:String::from("0"), target:String::from("0")};
    let cell8=Cell {id:String::from("minnie-8"), text:String::from("I'm an edge"), 
                          tooltip:String::from("I'm Minnie"),geometry:g10,cell_type:CellType::EDGE_WITH_LABEL,
                          source:String::from("minnie-4"), target:String::from("minnie-6")};
    let cell9=Cell {id:String::from("minnie-9"), text:String::from("I'm an edge"), 
                          tooltip:String::from("I'm Minnie"),geometry:g11,cell_type:CellType::EDGE_WITH_LABEL,
                          source:String::from("minnie-4"), target:String::from("minnie-7")};

    let mut cell_list:Vec<Cell>=Vec::new();
    
    //cell_list.push(cell1);
    cell_list.push(cell1);
    cell_list.push(cell2);
    cell_list.push(cell3);
    cell_list.push(cell4);
    cell_list.push(cell5);
    cell_list.push(cell6);
    cell_list.push(cell7);
    cell_list.push(cell8);
    cell_list.push(cell9); 
    


    let diagram=Diagram{id:String::from("FirstDiagram"),page_name:String::from("FirstPage"),
                         page_height:780,page_width:1200, cells: cell_list};


    //println!(">>> {:?}",Diagram::to_xml(diagram));

    let mut dia_list:Vec<Diagram>=Vec::new();

    dia_list.push(diagram);

    let doc= Document{host:String::from("app.diagrams.net"), repo_type:RepositoryType::DEVICE,modified:String::from("2022-12-08T22:27:57.684Z"),
           diagram_list:dia_list };
    
    println!(">>> {}",Document::to_xml(doc));

    //println!("{}",Cell::to_xml(&cell3));

}