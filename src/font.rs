
use libharu_sys as haru;
use document_inner::DocumentInner;
use std::rc::Rc;
use std::f32;

#[derive(Clone)]
pub struct Font {
    handle: haru::HPDF_Font,
    _doc: Rc<DocumentInner>
}


impl  Font {
    pub fn from_handle(handle: haru::HPDF_Font, doc: Rc<DocumentInner>) -> Font {
        Font{ handle: handle, _doc: doc}
    }
    
    pub fn get_ascent(&self) -> f32 {
        unsafe {
            haru::HPDF_Font_GetAscent(self.handle) as f32 / 1000.0
        }
    }
    
    pub fn get_descent(&self) -> f32 {
        unsafe {
            haru::HPDF_Font_GetAscent(self.handle) as f32 / 1000.0
        }
    }
    
    pub unsafe fn get_handle(&self) -> haru::HPDF_Page {
        self.handle
    }
    
    pub fn measure_width(&self, text: &str, size: f32) -> f32 {
        let mut actual_width: f32 = 0.0;
        unsafe {
            haru::HPDF_Font_MeasureText(
                self.handle,
                text.as_bytes().as_ptr(),
                text.as_bytes().len() as u32,
                f32::MAX, // width to fit in
                size,
                0.0, // char space
                0.0, // word space
                0, // word wrap off
                &mut actual_width
            );
        }
        actual_width
    }
}
