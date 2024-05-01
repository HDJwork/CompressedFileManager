#![allow(non_snake_case)]

pub use super::EType;

pub trait IPreviewedFile
{
    fn GetType(&self) -> EType;
    fn GetTmpPath(&self) -> String;
}

#[cfg(debug_assertions)]
pub struct DummyPreviewedFile{
    
}
#[cfg(debug_assertions)]
impl IPreviewedFile for DummyPreviewedFile{
    fn GetType(&self) -> EType{return EType::Image;}
    fn GetTmpPath(&self) -> String{return String::new();}
}