#![allow(non_snake_case)]

pub trait ICompressManager{
    fn Open(&mut self)->bool;
    fn IsOpen(&self)->bool;
    fn Close(&mut self)->bool;
    fn GetFileList(&mut self)->Vec<String>;
    fn Compress(&mut self, outputPath:&str, deleteFileList : Box<dyn std::iter::Iterator<Item = String>>)->bool;
}


#[cfg(debug_assertions)]
pub struct DummyCompressManager
{

}

#[cfg(debug_assertions)]
impl ICompressManager for DummyCompressManager
{
    fn IsOpen(&self)->bool   {   return false;   }
    fn Open(&mut self)->bool{    return false;
        
    }
    fn Close(&mut self)->bool
    {
        return false;

    }
    fn GetFileList(&mut self)->Vec<String>
    {
        return vec!["FromDummyClass".to_string()]

    }
    fn Compress(&mut self, _outputPath:&str, _deleteFileList : Box<dyn std::iter::Iterator<Item = String>>)->bool
    {
        return false;
    }
}