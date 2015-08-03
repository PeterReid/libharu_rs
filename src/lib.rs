extern crate libharu_sys;
extern crate libc;

mod document;
mod document_inner;
mod error;
mod font;
mod page;
mod page_layout;

pub use document::Document;
pub use error::{Code, Error, Result};
pub use font::Font;
pub use page::{Page, LineCap, LineJoin, Point, Size};


#[test]
fn simple() {
    use std::fs::File;
    fn go() -> Result<()> {
        let doc = try!(Document::new());
        
        let mut page1 = try!(doc.add_page());
        try!(page1.set_height(220.0));
        try!(page1.set_width(200.0));
        
        try!(page1.set_rgb_fill(1.0, 0.0, 0.0));
        try!(page1.move_to(Point::new(100.0, 100.0)));
        try!(page1.line_to(Point::new(100.0, 180.0)));
        try!(page1.arc(Point::new(100.0, 100.0), Point::new(80.0, 0.0), 360.0 * 0.45));
        try!(page1.line_to(Point::new(100.0, 100.0)));
        try!(page1.fill());
        
        /*let font = try!(doc.get_ttf_font(File::open("Roboto-Regular.ttf").unwrap()));
        
        let to_print = "35° 44.23' with some";
        try!(page1.set_rgb_fill(0.0, 0.0, 0.0));
        try!(page1.set_font_and_size(&font, 14.0));
        try!(page1.begin_text());
        try!(page1.text_out(Point::new(40.0, 40.0), to_print));
        try!(page1.end_text());
        
        let width = font.measure_width(to_print, 14.0);
        println!("Width = {}", width);
        
        try!(page1.move_to(Point::new(40.0, 40.0)));
        try!(page1.line_to(Point::new(40.0 + width, 40.0)));
        try!(page1.stroke());*/
        
        let mut out = File::create("2.pdf").unwrap();
        try!(doc.save(&mut out));
        
        
        
        Ok( () )
    }
    
    go().unwrap();
    panic!("To see printlns");
}

