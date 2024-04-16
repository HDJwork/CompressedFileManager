#![allow(non_snake_case)]

struct PreviewedFile_Imange{

}

impl IPreviewedFile for PreviewedFile_Imange
{
    fn GetType(&self) -> EType{return EType::Image;}
}