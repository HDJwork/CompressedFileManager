#![allow(non_snake_case)]

use crate::CompressManager::ICompressManager::ICompressManager;
use crate::CompressManager::MinizWrapperDllObj::MinizWrapperDllObj;

pub struct CompressManagerImpl
{
    bOpen : bool,
    dllobj : MinizWrapperDllObj
}
impl CompressManagerImpl{
    pub fn new()->CompressManagerImpl
    {
        CompressManagerImpl{
            bOpen:false,
            dllobj:MinizWrapperDllObj::new(),
        }
    }
}
impl ICompressManager for CompressManagerImpl
{
    fn IsOpen(&self)->bool   {   return self.bOpen;   }
    fn Open(&self)->bool
    {
        //T.B.D need to work
        return false;
    }
    fn Close(&self)->bool
    {
        //T.B.D need to work
        return false;

    }
    fn GetFileList(&self)->Vec<String>
    {
        
        //T.B.D need to work
        //dllobj.read()
        return Vec::new();

    }
    fn Compress(&self)->bool
    {
        //T.B.D need to work
        return false;

    }
}