
use libharu_sys as haru;
use std::ffi::CString;
use std::ptr;
use std::io::{Read, Write, Seek, SeekFrom};
use std::mem::{transmute, forget};
use std::slice;
use std::boxed::Box;

use error::{Code, Error, Result};
use font::Font;
use page::Page;
use page_layout::PageLayout;
use std::ops::DerefMut;

pub struct Document {
    handle: haru::HPDF_Doc,
}

impl Document {
    pub fn new() -> Result<Document> {
        let handle = try!(Error::check_non_null(unsafe { haru::HPDF_New(None, ptr::null_mut()) }));
        
        try!( Error::from_status(unsafe { haru::HPDF_UseUTFEncodings(handle) } ) );
        
        
        Ok(Document {
            handle: handle
        })
    }
    
    pub fn set_pages_configuration(&mut self, page_per_pages: u32) -> Result<()> {
        Error::from_status( unsafe { haru::HPDF_SetPagesConfiguration(self.handle, page_per_pages ) })
    }
    
    pub fn set_page_layout(&mut self, layout: PageLayout) -> Result<()> {
        let layout_code = layout.as_int();
        Error::from_status( unsafe { haru::HPDF_SetPageLayout(self.handle, layout_code ) })
    }
    
    pub fn get_page_layout(&self) -> Option<PageLayout> {
        PageLayout::from_int( unsafe { haru::HPDF_GetPageLayout(self.handle) } )
    }
    
    pub fn add_page<'a>(&'a self) -> Result<Page<'a>> {
        let page = try!(Error::check_non_null(unsafe { haru::HPDF_AddPage(self.handle) }));
        
        Ok(Page::from_handle(page))
    }
    
    pub fn insert_page<'a>(&'a self, target: Page<'a>) -> Result<Page<'a>> {
        let page = try!(Error::check_non_null(unsafe { haru::HPDF_InsertPage(self.handle, target.get_handle()) }));
        
        Ok(Page::from_handle(page))
    }
    
    /*pub fn get_font(&self, name: &str) -> Result<Font> {
        let chrs = try!(CString::new(name));
        let encoder_name = try!(CString::new("UTF-8"));
        
        let encoder = unsafe { haru::HPDF_GetEncoder(self.handle, encoder_name.as_ptr()) };
        println!("Encoder = {:?} {}", encoder, encoder==ptr::null_mut());
        
        let font_handle = unsafe { haru::HPDF_GetFont(self.handle, chrs.as_ptr(), encoder_name.as_ptr()) };
        if font_handle == ptr::null_mut() {
            println!("It returned null");
            try!(Error::from_status( unsafe { haru::HPDF_GetError(self.handle) } ) );
            return Error::new_err(Code::Unknown);
        }
        
        Ok(Font::from_handle(font_handle))
    }*/
    
    pub fn get_ttf_font2<R: Read+Seek>(&self, r: R) -> Result<Font> {
        extern "C" fn read<R: Read+Seek>(stream: haru::HPDF_Stream, ptr: *mut haru::HPDF_BYTE, size: *mut haru::HPDF_UINT) -> haru::HPDF_STATUS {
            let r : &mut TellingReader<R> = unsafe { transmute( (*stream).attr ) };
            let buf: &mut [u8] = unsafe { slice::from_raw_parts_mut(ptr, *size as usize) };
            
            let mut read_total = 0;
            while read_total < buf.len() {
                let read_len = match r.reader.read(&mut buf[read_total..]) {
                    Ok(0) => { 
                        unsafe { *size = read_total as haru::HPDF_UINT };
                        println!("Read to EOF");
                        return 0x1058 /* EOF */; 
                    }
                    Ok(read_len) => read_len,
                    Err(_) => { 
                        unsafe { *size = 0; }
                        return 0x1016; 
                    },
                };
                
                read_total += read_len;
            }
            
            r.pos += read_total as u64;
            
            return 0;
        }
        
        extern "C" fn seek<R: Read+Seek>(stream: haru::HPDF_Stream, pos: haru::HPDF_INT, mode: haru::HPDF_WhenceMode) -> haru::HPDF_STATUS {
            let r : &mut TellingReader<R> = unsafe { transmute( (*stream).attr ) };
            
            let res = r.reader.seek(match mode {
                haru::HPDF_SEEK_CUR => SeekFrom::Current(pos as i64),
                haru::HPDF_SEEK_END => SeekFrom::End(pos as i64),
                haru::HPDF_SEEK_SET | _ => SeekFrom::Start(pos as u64),
            });
            
            if let Ok(sought_to) = res {
                r.pos = sought_to;
                0
            } else {
                0x1016
            }
        }
        extern "C" fn tell<R: Read+Seek>(stream: haru::HPDF_Stream) -> haru::HPDF_INT32 {
            let r : &mut TellingReader<R> = unsafe { transmute( (*stream).attr ) };
            r.pos as haru::HPDF_INT32
        }
        extern "C" fn size<R: Read+Seek>(stream: haru::HPDF_Stream) -> haru::HPDF_UINT32 {
            let r : &mut TellingReader<R> = unsafe { transmute( (*stream).attr ) };
            let saved_pos = r.pos;
            let ret = r.reader.seek(SeekFrom::End(0)).ok().unwrap_or(0);
            if r.reader.seek(SeekFrom::Start(saved_pos)).is_err() {
                // Oops -- we failed to seek back to where we started.
                return 0;
            }
            ret as haru::HPDF_UINT32
        }
        extern "C" fn free<R: Read+Seek>(stream: haru::HPDF_Stream) {
            let r : Box<TellingReader<R>> = unsafe { transmute( (*stream).attr ) };
            drop(r)
        }
        
        struct TellingReader<R: Read+Seek> {
            reader: R,
            pos: u64
        }
        impl<R: Read+Seek> Drop for TellingReader<R> {
            fn drop(&mut self) {
                println!("Dropping the TellingReader");
            }
        }
        
        let mmgr = unsafe { haru::HPDF_GetMMgr(self.handle) };
        
        let mut tr = Box::new(TellingReader{
            reader: r,
            pos: 0
        });
        
        let stream = unsafe { haru::HPDF_CallbackReader_New(
            mmgr, 
            Some(read::<R>),
            Some(seek::<R>),
            Some(tell::<R>),
            Some(size::<R>),
            Some(free::<R>),
            tr.deref_mut() as *mut TellingReader<R> as *mut ::libc::c_void
        ) };
        
        forget(tr);
        
        // The font takes ownership of the stream
        let name = unsafe { haru::HPDF_LoadTTFontFromStream(self.handle, stream, 1, ptr::null()) };
         
        if name == ptr::null() {
            try!(Error::from_status( unsafe { haru::HPDF_GetError(self.handle) } ) );
            return Error::new_err(Code::Unknown);
        }
        
        let font_handle = unsafe { haru::HPDF_GetFont(self.handle, name, b"UTF-8".as_ptr() as *const i8) };
        if font_handle == ptr::null_mut() {
            try!(Error::from_status( unsafe { haru::HPDF_GetError(self.handle) } ) );
            return Error::new_err(Code::Unknown);
        }
        
        Ok(Font::from_handle(font_handle))
    }
    
    pub fn get_ttf_font(&self, file_name: &str) -> Result<Font> {
        let file_name_buf = try!(CString::new(file_name));
        let name = unsafe { haru::HPDF_LoadTTFontFromFile(self.handle, file_name_buf.as_ptr(), 1) };
        if name == ptr::null() {
            println!("HPDF_LoadTTFontFromFile null");
            try!(Error::from_status( unsafe { haru::HPDF_GetError(self.handle) } ) );
            return Error::new_err(Code::Unknown);
        }
        
        let encoder_name = try!(CString::new("UTF-8"));
        
        let encoder = unsafe { haru::HPDF_GetEncoder(self.handle, encoder_name.as_ptr()) };
        println!("Encoder = {:?} {}", encoder, encoder==ptr::null_mut());
        
        let font_handle = unsafe { haru::HPDF_GetFont(self.handle, name, encoder_name.as_ptr()) };
        if font_handle == ptr::null_mut() {
            println!("HPDF_GetFont returned null");
            try!(Error::from_status( unsafe { haru::HPDF_GetError(self.handle) } ) );
            return Error::new_err(Code::Unknown);
        }
        
        Ok(Font::from_handle(font_handle))
    }

    pub fn save<W: Write>(&self, w: &mut W) -> Result<()> {
        extern "C" fn write_fn<W: Write>(stream: haru::HPDF_Stream, ptr: *const haru::HPDF_BYTE, size: haru::HPDF_UINT) -> haru::HPDF_STATUS {
            let w : &mut W = unsafe { transmute( (*stream).attr ) };
            let buf: &[u8] = unsafe { slice::from_raw_parts(ptr, size as usize) };
            
            match w.write_all(buf) {
                Ok( () ) => 0,
                Err( _ ) => 0x1016, // file io error
            }
        }
        
        let mmgr = unsafe { haru::HPDF_GetMMgr(self.handle) };
        let stream = unsafe { haru::HPDF_CallbackWriter_New(mmgr, Some(write_fn::<W>), w as *mut W as *mut ::libc::c_void) };
        let err = unsafe { haru::HPDF_SaveToExternalStream(self.handle, stream) };
        unsafe { haru::HPDF_Stream_Free(stream); }
        
        Error::from_status(err)
    }
}

impl Drop for Document {
    fn drop(&mut self) {
        unsafe { haru::HPDF_Free(self.handle); }
    }
}
