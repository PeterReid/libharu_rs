
use libharu_sys as haru;
use document_inner::DocumentInner;
use std::rc::Rc;

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
}
