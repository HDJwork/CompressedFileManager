#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::*;

#[derive(Clone)] 
pub struct PreviewedFile_Image{
    tmpPath : String,
}

impl PreviewedFile_Image{
    pub fn new(tmpPath:&str)->PreviewedFile_Image{PreviewedFile_Image{tmpPath:String::from(tmpPath)}}
}

impl IPreviewedFile for PreviewedFile_Image
{
    fn GetType(&self) -> EType{return EType::Image;}
    fn GetTmpPath(&self) -> String{return self.tmpPath.clone();}
    //fn GetSize(&self) -> usize{return std::mem::size_of::<PreviewedFile_Image>();}
    //unsafe fn Overwrite(&self,rawTarget:*mut u8) {std::ptr::write(rawTarget as * mut Self,self.clone()); }

}