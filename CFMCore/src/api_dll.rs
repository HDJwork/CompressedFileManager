#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


//CompressedFileManager.rs
//use std::ffi::*;
use std::alloc::{alloc, Layout};

//use crate::previewed_file::IPreviewedFile;
use crate::utility_c::Type_C::*;
use crate::utility_c::Utility_C;

use super::compressed_file::CompressedFile;

struct PreviewResult{
    pub ptr_compressedFile:C_PTR ,
    pub file : String,
}

#[no_mangle]
pub extern "C" fn Startup(){
    crate::singleton_manager::startup();
}

#[no_mangle]
pub extern "C" fn Cleanup(){
    crate::singleton_manager::cleanup();
}

#[no_mangle]
pub extern "C" fn Open(out_ptr_compressedFile: C_PTR, path_c: C_STR)->C_BOOL{
    
    let path = Utility_C::ptr_to_String(path_c);
    
    // 메모리 할당
    let layout = Layout::new::<CompressedFile>();
    let compressedFile:&mut CompressedFile =unsafe {
        let ptr=  alloc(layout) ;
        *out_ptr_compressedFile = ptr as u64;
        let targetPtr =ptr as *mut CompressedFile;
        std::ptr::write(targetPtr, CompressedFile::new_without_open(path.as_str()));
        targetPtr.as_mut().unwrap()
    };
    match compressedFile.Open(){
        true=>{return C_TRUE;},
        false=>{return C_FALSE;}
    }
}

#[no_mangle]
pub extern "C" fn Close(ptr_compressedFile: C_PTR)->C_BOOL{
    unsafe{
        //for delete
        let ptr = Utility_C::ptr_to_ptr::<CompressedFile>(ptr_compressedFile);
        let _ = Box::from_raw(ptr);

        *ptr_compressedFile=0;
    }
    return C_TRUE;
}

#[no_mangle]
pub extern "C" fn GetFileCount(ptr_compressedFile: C_PTR)->C_INT{
    let compressedFile = Utility_C::ptr_to_ref::<CompressedFile>(ptr_compressedFile);
    return compressedFile.GetFileList().len() as C_INT;
}

#[no_mangle]
pub extern "C" fn GetFile(ptr_compressedFile: C_PTR, index:C_INT, out_buff:C_STR, buff_size:C_INT)->C_BOOL{
    let compressedFile = Utility_C::ptr_to_ref::<CompressedFile>(ptr_compressedFile);

    let fileList=compressedFile.GetFileList();
    if index >= fileList.len() as C_INT{
        return C_FALSE;
    }

    match Utility_C::string_Write_to_CBuffer(fileList[index as usize].clone(),out_buff,buff_size){
        true=>{return C_TRUE;},
        false=>{return C_FALSE;}
    }
}

#[no_mangle]
pub extern "C" fn DeleteFile(ptr_compressedFile: C_PTR, file_c:C_STR)->C_BOOL{
    let compressedFile = Utility_C::ptr_to_ref::<CompressedFile>(ptr_compressedFile);

    let file=Utility_C::ptr_to_String(file_c);
    match compressedFile.DeleteFile(file.as_str()){
        true=>{return C_TRUE;},
        false=>{return C_FALSE;}
    }
}

#[no_mangle]
pub extern "C" fn RevertDeleteFile(ptr_compressedFile: C_PTR, file_c:C_STR)->C_BOOL{
    let compressedFile = Utility_C::ptr_to_ref::<CompressedFile>(ptr_compressedFile);

    let file=Utility_C::ptr_to_String(file_c);
    match compressedFile.RevertDeletedFile(file.as_str()){
        true=>{return C_TRUE;},
        false=>{return C_FALSE;}
    }
}

#[no_mangle]
pub extern "C" fn IsChanged(ptr_compressedFile: C_PTR)->C_BOOL{
    let compressedFile = Utility_C::ptr_to_ref::<CompressedFile>(ptr_compressedFile);

    match compressedFile.IsChanged(){
        true=>{return C_TRUE;},
        false=>{return C_FALSE;}
    }
}

#[no_mangle]
pub extern "C" fn Recompress(ptr_compressedFile: C_PTR, resultPath_c:C_STR)->C_BOOL{
    let compressedFile = Utility_C::ptr_to_ref::<CompressedFile>(ptr_compressedFile);

    let resultPath=Utility_C::ptr_to_String(resultPath_c);
    match compressedFile.Recompress(resultPath.as_str()){
        true=>{return C_TRUE;},
        false=>{return C_FALSE;}
    }
}


#[no_mangle]
pub extern "C" fn PreviewFile(ptr_compressedFile: C_PTR,out_ptr_previewedFile: C_PTR, file_c:C_STR)->C_BOOL{
    let compressedFile = Utility_C::ptr_to_ref::<CompressedFile>(ptr_compressedFile);

    let file=Utility_C::ptr_to_String(file_c);

    let layout = Layout::new::<PreviewResult>();
    unsafe{
        let ptr= alloc(layout) ;
        *out_ptr_previewedFile = ptr as u64;
        let targetPtr = ptr as *mut PreviewResult;
        std::ptr::write(targetPtr,PreviewResult{ptr_compressedFile:ptr_compressedFile, file:file.clone()});
    }
    
    match compressedFile.PreviewFile(file.as_str()){
        Ok(_previewedFile)=>{return C_TRUE;},
        Err(_)=>{return C_FALSE;}
    }
}

#[no_mangle]
pub extern "C" fn Preview_Release(ptr_previewedFile: C_PTR){
    unsafe{
        //for delete
        let ptr = Utility_C::ptr_to_ptr::<PreviewResult>(ptr_previewedFile);
        let _ = Box::from_raw(ptr);

        *ptr_previewedFile=0;
    }
}

#[no_mangle]
pub extern "C" fn Preview_GetType(ptr_previewedFile: C_PTR)->C_INT{
    let previewResult = Utility_C::ptr_to_ref::<PreviewResult>(ptr_previewedFile);
    let compressedFile = Utility_C::ptr_to_ref::<CompressedFile>(previewResult.ptr_compressedFile);

    use crate::custom_type::EType;
    
    match compressedFile.PreviewFile(previewResult.file.as_str()){
        Ok(previewedFile)=>{
            return previewedFile.GetType() as C_INT;
        },
        Err(_)=>{return EType::ERROR as C_INT;}
    }
}

#[no_mangle]
pub extern "C" fn Preview_GetTmpPath(ptr_previewedFile: C_PTR, out_buff:C_STR, buff_size:C_INT)->C_BOOL{
    let previewResult = Utility_C::ptr_to_ref::<PreviewResult>(ptr_previewedFile);
    let compressedFile = Utility_C::ptr_to_ref::<CompressedFile>(previewResult.ptr_compressedFile);
    
    match compressedFile.PreviewFile(previewResult.file.as_str()){
        Ok(previewedFile)=>{
            match Utility_C::string_Write_to_CBuffer(previewedFile.GetTmpPath(),out_buff,buff_size){
                true=>{return C_TRUE;},
                false=>{return C_FALSE;}
            }
        },
        Err(_)=>{return C_FALSE;}
    }
    
}

