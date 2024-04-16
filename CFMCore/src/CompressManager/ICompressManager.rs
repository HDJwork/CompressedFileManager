#![allow(non_snake_case)]

pub trait ICompressManager{
    fn Open(&self)->bool;
    fn IsOpen(&self)->bool;
    fn Close(&self)->bool;
    fn GetFileList(&self)->Vec<String>;
    fn Compress(&self)->bool;
}


#[cfg(debug_assertions)]
pub struct DummyCompressManager
{

}

#[cfg(debug_assertions)]
impl ICompressManager for DummyCompressManager
{
    fn IsOpen(&self)->bool   {   return false;   }
    fn Open(&self)->bool{    return false;
        
    }
    fn Close(&self)->bool
    {
        return false;

    }
    fn GetFileList(&self)->Vec<String>
    {
        return vec!["FromDummyClass".to_string()]

    }
    fn Compress(&self)->bool
    {
        return false;
    }
}