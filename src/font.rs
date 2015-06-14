
use libharu_sys as haru;
use std::marker::PhantomData;

#[derive(Copy, Clone)]
pub struct Font<'a> {
    handle: haru::HPDF_Font,
    marker: PhantomData<&'a i32>,
}


impl <'a> Font<'a> {
    pub fn from_handle(handle: haru::HPDF_Font) -> Font<'a> {
        Font{ handle: handle, marker: PhantomData }
    }
    
    pub unsafe fn get_handle(&self) -> haru::HPDF_Page {
        self.handle
    }
}
