#![allow(non_snake_case)]


//use crate::utility_c::Utility_C;

pub use super::EType;

pub trait IPreviewedFile
{
    fn GetType(&self) -> EType;
    fn GetTmpPath(&self) -> String;
    //fn GetSize(&self) -> usize;
    //unsafe fn Overwrite(&self,rawTarget:*mut u8) ;
}

#[cfg(debug_assertions)]
#[derive(Clone)] 
pub struct DummyPreviewedFile{
    
}
#[cfg(debug_assertions)]
impl IPreviewedFile for DummyPreviewedFile{
    fn GetType(&self) -> EType{return EType::Image;}
    fn GetTmpPath(&self) -> String{return String::new();}
    //fn GetSize(&self) -> usize{return std::mem::size_of::<DummyPreviewedFile>();}
    //unsafe fn Overwrite(&self,rawTarget:*mut u8) {std::ptr::write(rawTarget as * mut Self,self.clone()); }
}
