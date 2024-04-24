#![allow(non_snake_case)]

use crate::CompressManager::ICompressManager::ICompressManager;

pub struct CompressManagerImpl
{
    bOpen : bool,

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
        return Vec::new();

    }
    fn Compress(&self)->bool
    {
        //T.B.D need to work
        return false;

    }
}