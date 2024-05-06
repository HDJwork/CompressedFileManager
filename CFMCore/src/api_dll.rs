#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


//CompressedFileManager.rs
//use std::ffi::*;
use std::alloc::{alloc, dealloc, Layout};

use crate::utility_c::Type_C::*;
use crate::utility_c::Utility_C;

use super::compressed_file::CompressedFile;

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
    
    let layout = Layout::new::<CompressedFile>();

    // 메모리 할당
    let compressedFile:&mut CompressedFile =unsafe {
        let ptr=  alloc(layout) ;
        *out_ptr_compressedFile = ptr as u64;
        let path = Utility_C::ptr_to_String(path_c);
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
pub extern "C" fn Close(compressedFile: C_PTR)->C_BOOL{
    
    let layout = Layout::new::<CompressedFile>();
    unsafe{
        let ptr:*mut u8 = *compressedFile as *mut u8;   
        dealloc(ptr, layout);
        *compressedFile=0;
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
