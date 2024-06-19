#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use crate::utility_c::Type_C::*;
use crate::utility_c::Utility_C;

//use std::sync::{Arc,Mutex};
//use std::cell::OnceCell;

//#[macro_use]
//extern crate lazy_static;
//use lazy_static::lazy_static;

//pub type MinizWrapperDllObj_Static = &'static mut MinizWrapperDllObj;

//Function Type
type FnType_MINIZ_LIB_Read = unsafe extern "stdcall" fn(result: C_PTR, filename:C_STR)-> C_BOOL;
type FnType_MINIZ_LIB_Read_Result_Release = unsafe extern "stdcall" fn(result: C_PTR);
type FnType_MINIZ_LIB_Read_Result_GetErrorCode = unsafe extern "stdcall" fn(result: C_PTR)-> C_INT;
type FnType_MINIZ_LIB_Read_Result_GetCount = unsafe extern "stdcall" fn(result: C_PTR) -> C_INT;
type FnType_MINIZ_LIB_Read_Result_GetFileName = unsafe extern "stdcall" fn(result: C_PTR, index: C_INT, outputBuff:C_STR, outputBuffCount: C_INT) -> C_BOOL;

type FnType_MINIZ_LIB_Recompress = unsafe extern "stdcall" fn(src: C_STR, output: C_STR, passingList: C_STRS, noOfPassingList: C_INT) -> C_BOOL;

type FnType_MINIZ_LIB_Preview = unsafe extern "stdcall" fn(result: C_PTR,readResult: C_PTR, filename:C_STR)-> C_BOOL;
type FnType_MINIZ_LIB_Preview_Result_Release = unsafe extern "stdcall" fn(result: C_PTR);
type FnType_MINIZ_LIB_Preview_Result_GetErrorCode = unsafe extern "stdcall" fn(result: C_PTR)-> C_INT;
type FnType_MINIZ_LIB_Preview_Result_GetFilePath = unsafe extern "stdcall" fn(result: C_PTR,outputBuff:C_STR, outputBuffCount: C_INT) -> C_BOOL;


//---------------------------------------------------------- Struct --------------------------------------------------------------------
//#[derive(Copy, Clone)]
pub struct MinizWrapperDllObj
{
    dll_handle :usize,

    pub read : FnType_MINIZ_LIB_Read,
    pub readResult_Release : FnType_MINIZ_LIB_Read_Result_Release,
    pub readResult_GetErrorCode : FnType_MINIZ_LIB_Read_Result_GetErrorCode,
    pub readResult_GetCount : FnType_MINIZ_LIB_Read_Result_GetCount,
    pub readResult_GetFileName : FnType_MINIZ_LIB_Read_Result_GetFileName,
    pub recompress : FnType_MINIZ_LIB_Recompress,
    pub preview : FnType_MINIZ_LIB_Preview,
    pub previewResult_Release : FnType_MINIZ_LIB_Preview_Result_Release,
    pub previewResult_GetErrorCode : FnType_MINIZ_LIB_Preview_Result_GetErrorCode,
    pub previewResult_GetFilePath : FnType_MINIZ_LIB_Preview_Result_GetFilePath,
}

impl Drop for MinizWrapperDllObj
{
    fn drop(&mut self){
        use winapi::um::libloaderapi::FreeLibrary;   
        unsafe {
            let dll_handle :*mut winapi::shared::minwindef::HINSTANCE__ = std::mem::transmute(self.dll_handle);
            FreeLibrary(dll_handle); 
        }
        println!("DLL Unload Success!");
    }
}

static mut INSTANCE : Option<&'static mut MinizWrapperDllObj> = None;

impl MinizWrapperDllObj{
    //singleton
    pub fn instance() -> &'static mut MinizWrapperDllObj {
        unsafe {
            match INSTANCE {
                Option::Some(ref mut manager) => *manager,
                Option::None => {
                    println!("new instance!");
                    let manager_box = Box::new(Self::new_Impl());
                    let manager_raw = Box::into_raw(manager_box);
                    INSTANCE = Some(&mut *manager_raw);
                    &mut *manager_raw
                }
            }
        }
    }

    pub fn destroy() {
        unsafe {
            #[allow(static_mut_refs)]
            if let Some(raw) = std::mem::replace(&mut INSTANCE, None) {
                let _ = Box::from_raw(raw);
            }
        }
    }

    fn new_Impl() -> MinizWrapperDllObj{

        use std::os::windows::ffi::OsStrExt;
        use std::ffi::OsStr;
        //use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress, LoadLibraryW, FreeLibrary};
        use winapi::um::libloaderapi::LoadLibraryW;   
        
    
        // DLL 파일 경로
        let dll_filename = "miniz_DLL.dll";
        let mut dll_handle :*mut winapi::shared::minwindef::HINSTANCE__  = std::ptr::null_mut();
        let mut dll_path_list: Vec<String> = Vec::new() ;
        #[cfg(debug_assertions)]
        {
            dll_path_list.push(String::from("../ref/debug/")+dll_filename);
            dll_path_list.push(String::from("ref/")+dll_filename);
            dll_path_list.push(String::from(dll_filename));
        }
        #[cfg(not(debug_assertions))]
        {
            dll_path_list.push(String::from("ref/")+dll_filename);
            dll_path_list.push(String::from("../ref/release/")+dll_filename);
            dll_path_list.push(String::from(dll_filename));
        }
        let mut bSuccess = false;
        for path in dll_path_list{
            // DLL 파일 경로를 Wide 문자열로 변환
            let dll_path_wide: Vec<u16> = OsStr::new(path.as_str())
                .encode_wide()
                .chain(std::iter::once(0))
                .collect();

            dll_handle = unsafe { LoadLibraryW(dll_path_wide.as_ptr()) };
            if !dll_handle.is_null() {
                bSuccess = true;
                break;
            }
        }
        if !bSuccess {
            println!("DLL Load Fail!");
            panic!("DLL Load Fail!");
        }

        use Utility_C::getFunction as getFunc;
        let retval =  unsafe {
            MinizWrapperDllObj{
                dll_handle:std::mem::transmute(dll_handle),
                read 
                : getFunc("MINIZ_LIB_Read_UTF8",dll_handle).unwrap(),
                readResult_Release 
                : getFunc("MINIZ_LIB_Read_Result_Release", dll_handle).unwrap(),
                readResult_GetErrorCode 
                : getFunc("MINIZ_LIB_Read_Result_GetErrorCode",dll_handle).unwrap(),
                readResult_GetCount 
                : getFunc("MINIZ_LIB_Read_Result_GetCount",dll_handle).unwrap(),
                readResult_GetFileName 
                : getFunc("MINIZ_LIB_Read_Result_GetFileName_UTF8",dll_handle).unwrap(),
                recompress
                : getFunc("MINIZ_LIB_Recompress_UTF8",dll_handle).unwrap(),
                preview 
                : getFunc("MINIZ_LIB_Preview_UTF8",dll_handle).unwrap(),
                previewResult_Release 
                : getFunc("MINIZ_LIB_Preview_Result_Release",dll_handle).unwrap(),
                previewResult_GetErrorCode
                : getFunc("MINIZ_LIB_Preview_Result_GetErrorCode",dll_handle).unwrap(),
                previewResult_GetFilePath
                : getFunc("MINIZ_LIB_Preview_Result_GetTempFilePath_UTF8",dll_handle).unwrap(),
            
            }
        };
        println!("DLL Load Success!");

        return retval;
    }
   
   
}

    

//---------------------------------------------------------- Utility --------------------------------------------------------------------
