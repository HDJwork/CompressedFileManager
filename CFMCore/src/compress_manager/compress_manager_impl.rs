#![allow(non_snake_case)]
use super::i_compress_manager::*;
use std::iter::Iterator;
use std::collections::HashMap;

use crate::utility_c::Type_C::{
    C_HANDLE,
    C_CHAR,
    C_STR,
    C_CSTRING,
    C_HANDLE_NULL,
    C_TRUE,
    C_FALSE,
    C_BUFFER_MAX,
};
use crate::utility_c::Utility_C::{
    str_to_CString,
    handle_to_ptr,
    vec_to_String,
};
use crate::compress_manager::miniz_wrapper_dll_obj::MinizWrapperDllObj;

struct PreviewResult{
    previewHandle : C_HANDLE,
    previewedFile:Box<dyn IPreviewedFile>,
}
pub struct CompressManagerImpl
{
    path : String,
    dllobj :  &'static mut MinizWrapperDllObj,
    readHandle : C_HANDLE,
    previewList : HashMap<String,PreviewResult>
}

impl CompressManagerImpl{
    pub fn new(path : &str)->CompressManagerImpl
    {
        CompressManagerImpl{
            path : path.to_string(),
            dllobj : MinizWrapperDllObj::instance(),
            readHandle : C_HANDLE_NULL,
            previewList : HashMap::new(),
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

    fn Open(&mut self)->bool    {
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

    fn Close(&mut self)->bool    {
        if !Self::IsOpen(&self) { return false; }
        for preview in &mut self.previewList{
            println!("[CompressManagerImpl]Close => previewHandle{}",preview.1.previewHandle);
            unsafe {(self.dllobj.previewResult_Release)(handle_to_ptr(&mut preview.1.previewHandle));}
        }
        self.previewList.clear();
        println!("[CompressManagerImpl]Close => readHandle{}",self.readHandle);
        unsafe {(self.dllobj.readResult_Release)(handle_to_ptr(&mut self.readHandle));}
        return self.readHandle == C_HANDLE_NULL;

    }

    fn GetFileList(&mut self)->Vec<String>    {
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
                let fileName = vec_to_String(&buff);
                retval.push(fileName);
            }
        }
        
        return retval;
    }

    fn Compress(&mut self, outputPath:&str, deleteFileList : Box<dyn Iterator<Item = String>>)->bool    {
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

    fn PreviewFile(&mut self, file:&str)->Result<&Box<dyn IPreviewedFile>,String>    {

        use crate::previewed_file::previewed_file_builder;
        let key = String::from(file);
        if !self.previewList.contains_key(&key){
            let resultPair = unsafe{
                let mut handle :C_HANDLE = C_HANDLE_NULL;
                let ptr_read = handle_to_ptr(&mut self.readHandle);
                let ptr = handle_to_ptr(&mut handle);
                let path_C :C_CSTRING= str_to_CString(file);
                if (self.dllobj.preview)(ptr,ptr_read,path_C.as_ptr()) == C_FALSE{
                    let errorCode = (self.dllobj.previewResult_GetErrorCode)(ptr);
                    (self.dllobj.previewResult_Release)(ptr);
                    return Err(format!("previewResult fail, ErrorCode = {}",errorCode))
                }
                
                let mut buff : Vec<C_CHAR> = Vec::new();
                buff.resize(C_BUFFER_MAX as usize, 0);
                let result=(self.dllobj.previewResult_GetFilePath)(ptr,buff.as_ptr() as C_STR,C_BUFFER_MAX);
                if result == C_FALSE{
                    return Err(String::from("previewResult_GetFilePath fail!"))

                }
                (handle,vec_to_String(&buff))
            };
            match previewed_file_builder::buildPreviewedFile(resultPair.1.as_str()){
                Ok(previewedFile)=>{
                    self.previewList.insert(key.clone(),
                        PreviewResult{
                            previewedFile:previewedFile,
                            previewHandle:resultPair.0,
                        }
                    );
                }
                Err(e)=>{return Err(e)}
            }
        }
        
        return Ok(&self.previewList[&key].previewedFile);
    }
}