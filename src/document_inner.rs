use libharu_sys as haru;

pub struct DocumentInner {
    pub handle: haru::HPDF_Doc,
}

impl Drop for DocumentInner {
    fn drop(&mut self) {
        unsafe { haru::HPDF_Free(self.handle); }
    }
}
