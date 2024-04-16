#![allow(non_snake_case)]

use crate::Type::EType;

pub trait IPreviewedFile
{
    fn GetType(&self) -> EType;
}

#[cfg(debug_assertions)]
pub struct DummyPreviewedFile{
    
}
#[cfg(debug_assertions)]
impl IPreviewedFile for DummyPreviewedFile{
    fn GetType(&self) -> EType{return EType::Image;}
}