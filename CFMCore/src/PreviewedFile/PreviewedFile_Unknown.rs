#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use super::IPreviewedFile::*;

pub struct PreviewedFile_Unknown{
    tmpPath : String,
}

impl PreviewedFile_Unknown{
    pub fn new(tmpPath:&str)->PreviewedFile_Unknown{PreviewedFile_Unknown{tmpPath:String::from(tmpPath)}}
}

impl IPreviewedFile for PreviewedFile_Unknown
{
    fn GetType(&self) -> EType{return EType::Unknown;}
    fn GetTmpPath(&self) -> String{return self.tmpPath.clone();}
}