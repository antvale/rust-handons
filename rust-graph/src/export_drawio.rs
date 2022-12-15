use std::{io::{self, Read, Result}, fmt::format};
use rand::Rng;


#[derive(Debug)]
pub enum RepositoryType {
    DEVICE,
    GOOGLE_DRIVE,
    ONEDRIVE
}

#[derive(Debug)]
pub struct Diagram {
    pub id:String,
    pub page_name:String,
    pub page_width:u16,
    pub page_height:u16,
    pub cells: Vec<Cell>
}


#[derive(Debug)]
pub struct Document {
    pub host: String,
    pub repo_type: RepositoryType,
    pub modified: String,
    pub diagram_list: Vec<Diagram>
 
}


#[derive(Debug)]
pub struct Geometry {
    relative: u8,
    width: u16,
    height: u16,
    x: u16,
    y: u16
}

#[derive(Debug)]
pub enum CellType {
    EDGE,
    EDGE_WITH_LABEL,
    RECTANGLE,
    DIAMON,
    DOCUMENT
}

#[derive(Debug)]
pub struct Cell{
    pub id: String,
    pub cell_type: CellType, 
    pub text: String,
    pub tooltip: String,
    pub geometry: Geometry,
    pub target: String,
    pub source: String
}
impl Diagram {
    pub fn to_xml(diagram:Diagram) -> String{
        
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
    pub fn to_xml (doc: Document) -> String {
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
    pub fn get_default_geometry(cell_type:CellType) -> Geometry {

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

    pub fn to_xml (cell: &Cell) -> String {
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