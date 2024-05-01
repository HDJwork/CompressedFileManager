#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::*;

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
}