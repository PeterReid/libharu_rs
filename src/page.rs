
use libharu_sys as haru;
use std::ffi::CString;
use std::marker::PhantomData;

use error::{Error, Result};
use font::Font;

pub struct Page<'a> {
    handle: haru::HPDF_Page,
    marker: PhantomData<&'a i32>,
}

pub enum LineCap {
    ButtEnd = haru::HPDF_BUTT_END as isize,
    RoundEnd = haru::HPDF_ROUND_END as isize,
    ProjectingSquareEnd = haru::HPDF_PROJECTING_SCUARE_END as isize,
}

pub enum LineJoin {
    Miter = haru::HPDF_MITER_JOIN as isize,
    Round = haru::HPDF_ROUND_JOIN as isize,
    Bevel = haru::HPDF_BEVEL_JOIN as isize,
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Point {
        Point{ x: x, y: y }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size {
    pub fn new(width: f32, height: f32) -> Size {
        Size{ width: width, height: height }
    }
}


impl <'a> Page<'a> {
    pub fn from_handle(handle: haru::HPDF_Page) -> Page<'a> {
        Page{ handle: handle, marker: PhantomData }
    }
    
    pub unsafe fn get_handle(&self) -> haru::HPDF_Page {
        self.handle
    }
    
    pub fn set_width(&mut self, width: f32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetWidth(self.handle, width) })
    }
    
    pub fn set_height(&mut self, height: f32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetHeight(self.handle, height) })
    }
    
    pub fn set_line_width(&mut self, line_width: f32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetLineWidth(self.handle, line_width) })
    }
    
    pub fn set_line_cap(&mut self, line_cap: LineCap) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetLineCap(self.handle, line_cap as haru::HPDF_LineCap) })
    }
    
    pub fn set_line_join(&mut self, line_join: LineJoin) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetLineJoin(self.handle, line_join as haru::HPDF_LineJoin) })
    }
    
    pub fn set_miter_limit(&mut self, miter_limit: f32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetMiterLimit(self.handle, miter_limit) })
    }
    
    pub fn set_dash(&mut self, dash_pattern: &[u16], phase: u32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetDash(
            self.handle, 
            dash_pattern.as_ptr(), 
            dash_pattern.len() as u32, 
            phase) 
        })
    }
    
    pub fn set_flat(&mut self, flatness: f32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetFlat(self.handle, flatness) })
    }
    
    // Color operators
    
    pub fn set_gray_fill(&mut self, gray: f32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetGrayFill(self.handle, gray) })
    }
    
    pub fn set_gray_stroke(&mut self, gray: f32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetGrayStroke(self.handle, gray) })
    }
    
    pub fn set_rgb_fill(&mut self, r: f32, g: f32, b: f32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetRGBFill(self.handle, r, g, b) })
    }
    
    pub fn set_cmyk_stroke(&mut self, c: f32, m: f32, y: f32, k: f32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetCMYKStroke(self.handle, c, m, y, k) })
    }
    
    pub fn set_cmyk_fill(&mut self, c: f32, m: f32, y: f32, k: f32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetCMYKFill(self.handle, c, m, y, k) })
    }
    
    pub fn move_to(&mut self, point: Point) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_MoveTo(self.handle, point.x, point.y) })
    }
    
    pub fn line_to(&mut self, end: Point) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_LineTo(self.handle, end.x, end.y) })
    }
    
    pub fn curve_to(&mut self, outbound_control: Point, inbound_control: Point, end: Point) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_CurveTo(self.handle, 
            outbound_control.x, outbound_control.y, inbound_control.x, inbound_control.y, end.x, end.y)})
    }
    
    pub fn curve_to_2(&mut self, inbound_control: Point, end: Point) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_CurveTo2(self.handle, 
            inbound_control.x, inbound_control.y, end.x, end.y)})
    }
    
    pub fn curve_to_3(&mut self, outbound_control: Point, end: Point) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_CurveTo3(self.handle, 
            outbound_control.x, outbound_control.y, end.x, end.y)})
    }
    
    pub fn arc(&mut self, center: Point, ray_endpoint: Point, angle_degrees: f32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_Arc(self.handle, 
            center.x, center.y, ray_endpoint.x, ray_endpoint.y, angle_degrees)})
    }
    
    pub fn close_path(&mut self) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_ClosePath(self.handle) } )
    }
    
    pub fn rectangle(&mut self, lower_left: Point, size: Size) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_Rectangle(self.handle,
            lower_left.x, lower_left.y, size.width, size.height) } )
    }
    
    pub fn stroke(&mut self) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_Stroke(self.handle) } )
    }
    pub fn close_path_stroke(&mut self) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_ClosePathStroke(self.handle) } )
    }
    pub fn fill(&mut self) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_Fill(self.handle) } )
    }
    pub fn eofill(&mut self) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_Eofill(self.handle) } )
    }
    pub fn fill_stroke(&mut self) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_FillStroke(self.handle) } )
    }
    pub fn eofill_stroke(&mut self) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_EofillStroke(self.handle) } )
    }
    pub fn close_path_fill_stroke(&mut self) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_ClosePathFillStroke(self.handle) } )
    }
    pub fn close_path_eofill_stroke(&mut self) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_ClosePathEofillStroke(self.handle) } )
    }
    pub fn end_path(&mut self) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_EndPath(self.handle) } )
    }
    
    pub fn set_font_and_size(&mut self, font: Font, size: f32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_SetFontAndSize(self.handle, font.get_handle(), size) } )
    }
    
    
    pub fn begin_text(&mut self) ->  Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_BeginText(self.handle) } )
    }
    pub fn end_text(&mut self) ->  Result<()> {
        Error::from_status( unsafe { haru::HPDF_Page_EndText(self.handle) } )
    }
    pub fn text_out(&mut self, baseline_left: Point, text: &str) ->  Result<()> {
        let chrs = try!(CString::new(text));
        Error::from_status( unsafe { haru::HPDF_Page_TextOut(self.handle, baseline_left.x, baseline_left.y, chrs.as_ptr()) } )
    }
    
}