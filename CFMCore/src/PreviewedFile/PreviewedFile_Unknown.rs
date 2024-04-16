#![allow(non_snake_case)]

struct PreviewedFile_Unknown{

}

impl IPreviewedFile for PreviewedFile_Unknown
{
    fn GetType(&self) -> EType{return EType::Unknown;}
}