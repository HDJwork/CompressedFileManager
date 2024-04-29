#![allow(non_snake_case)]

use crate::CompressManager::ICompressManager::ICompressManager;
use crate::CompressManager::MinizWrapperDllObj::{
    C_HANDLE,
    C_CHAR,
    C_STR,
    C_CSTRING,
    C_HANDLE_NULL,
    C_TRUE,
    C_FALSE,
    C_BUFFER_MAX,
};
use crate::CompressManager::MinizWrapperDllObj::Utility::{
    str_to_CString,
    handle_to_ptr,
    to_string,
};
use crate::CompressManager::MinizWrapperDllObj::MinizWrapperDllObj;

pub struct CompressManagerImpl
{
    path : String,
    dllobj :  &'static mut MinizWrapperDllObj,
    readHandle : C_HANDLE,
}

impl CompressManagerImpl{
    pub fn new(path : &str)->CompressManagerImpl
    {
        CompressManagerImpl{
            path:path.to_string(),
            dllobj:MinizWrapperDllObj::instance(),
            readHandle:C_HANDLE_NULL,
        }
    }
}

impl Drop for CompressManagerImpl{
    fn drop(&mut self){
        self.Close();
    }
}

impl ICompressManager for CompressManagerImpl
{
    fn IsOpen(&self)->bool   {   return self.readHandle!=C_HANDLE_NULL;   }
    fn Open(&mut self)->bool
    {
        Self::Close(self);
        
        let result = unsafe{
            let path_C :C_CSTRING= str_to_CString(self.path.as_str());
            (self.dllobj.read)(handle_to_ptr(&mut self.readHandle),path_C.as_ptr())
        };
        if result == C_FALSE 
        {
            println!("[CompressManagerImpl]Open fail!");
            return false;
        }
        else if self.readHandle==C_HANDLE_NULL
        {
            println!("[CompressManagerImpl]read handle is null!");
            return false;
        }
        
        println!("[CompressManagerImpl]Open {}",self.readHandle);
        return true;
    }
    fn Close(&mut self)->bool
    {
        if !Self::IsOpen(&self) { return false; }

        println!("[CompressManagerImpl]Close {}",self.readHandle);
        unsafe {(self.dllobj.readResult_Release)(handle_to_ptr(&mut self.readHandle));}
        return self.readHandle == C_HANDLE_NULL;

    }
    fn GetFileList(&mut self)->Vec<String>
    {
        let mut retval :Vec<String> =Vec::new();
        if !Self::IsOpen(self)
        {
            println!("[CompressManagerImpl]not opened!");
            return retval;
        }
        let ptr = handle_to_ptr(&mut self.readHandle);
        let count =unsafe{(self.dllobj.readResult_GetCount)(ptr)};
        
        let mut buff : Vec<C_CHAR> = Vec::new();
        buff.resize(C_BUFFER_MAX as usize, 0);

        for i in 0..count{
            let result=unsafe {(self.dllobj.readResult_GetFileName)(ptr,i,buff.as_ptr() as C_STR,C_BUFFER_MAX)};
            if result != C_FALSE
            {
                let fileName = to_string(&buff);
                retval.push(fileName);
            }
        }
        
        return retval;
    }
    fn Compress(&mut self, outputPath:&str, deleteFileList : Box<dyn std::iter::Iterator<Item = String>>)->bool
    {
        let mut strTmpContiner :Vec<C_CSTRING>=Vec::new();
        let mut strContiner :Vec<C_STR>=Vec::new();
        for fileName in deleteFileList{
            strTmpContiner.push(str_to_CString(fileName.as_str()));
            strContiner.push(strTmpContiner.last().unwrap().as_ptr());
        }
        let count = strTmpContiner.len() as i32;
        
        let path_C :C_CSTRING= str_to_CString(self.path.as_str());
        let outputPath_C :C_CSTRING= str_to_CString(outputPath);
        let result = unsafe{(self.dllobj.recompress)(path_C.as_ptr(),outputPath_C.as_ptr(),strContiner.as_ptr(),count)};
        if result == C_TRUE{
            //T.B.D
            return true;
        }
        else {
            //T.B.D
            return false;
        }
    }
}