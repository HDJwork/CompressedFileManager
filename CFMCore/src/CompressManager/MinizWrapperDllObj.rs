#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::sync::{Arc,Mutex};
use std::cell::OnceCell;

//#[macro_use]
//extern crate lazy_static;
use lazy_static::lazy_static;
//---------------------------------------------------------- Type Definition --------------------------------------------------------------------
//C Type
type C_PTR = * mut std::ffi::c_ulonglong;
type C_BOOL = std::ffi::c_int;
type C_INT = std::ffi::c_int;
type C_CHAR = std::ffi::c_char;
type C_STR = * const C_CHAR;
type C_STRS = * const C_STR;

pub const C_FALSE : C_BOOL = 0;
#[allow(dead_code)]
pub const C_TRUE : C_BOOL = 1;

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
struct HandleContainer{
    dll_handle :usize,
}

#[derive(Copy, Clone)]
pub struct MinizWrapperDllObj
{
    //dll_handle :*mut winapi::shared::minwindef::HINSTANCE__,

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

impl Drop for HandleContainer
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

lazy_static!{
    static ref INSTANCE: Mutex<Option<(Box<HandleContainer>,MinizWrapperDllObj)>>=Mutex::new(None) ;
}



impl MinizWrapperDllObj
{
    pub fn new() -> MinizWrapperDllObj
    {
        let mut guard=INSTANCE.lock().unwrap();
        if (*guard).is_none()
        {
            *guard=Some(Self::new_Impl());
        }
        let tuple=(*guard).as_ref().unwrap();

        //let mut guard=INSTANCE.lock().unwrap();
        //return guard.unwrap();

        return tuple.1;
    }
    fn new_Impl() -> (Box<HandleContainer>,MinizWrapperDllObj){

        use std::os::windows::ffi::OsStrExt;
        use std::ffi::OsStr;
        //use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress, LoadLibraryW, FreeLibrary};
        use winapi::um::libloaderapi::LoadLibraryW;   
        
        // DLL 파일 경로
        let dll_path = "../ref/miniz_DLL.dll";
        
        // DLL 파일 경로를 Wide 문자열로 변환
        let dll_path_wide: Vec<u16> = OsStr::new(dll_path)
            .encode_wide()
            .chain(std::iter::once(0))
            .collect();

        //------------------------------------- DLL Load --------------------------------------
        // 
        let dll_handle = unsafe { LoadLibraryW(dll_path_wide.as_ptr()) };
        if dll_handle.is_null() {
            println!("DLL Load Fail!");
            panic!("DLL Load Fail!");
        }

        use Utility::getFunction as getFunc;
        let retval =  unsafe {
            MinizWrapperDllObj{
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
                : getFunc("MINIZ_LIB_Preview_Result_GetTempFilePath_UTF8",dll_handle).unwrap(),
                previewResult_GetFilePath
                : getFunc("MINIZ_LIB_Recompress_UTF8",dll_handle).unwrap(),
            
            }
        };
        println!("DLL Load Success!");

        return (Box::new(HandleContainer{dll_handle:unsafe {std::mem::transmute(dll_handle)}}), retval);
    }

}



//---------------------------------------------------------- Utility --------------------------------------------------------------------

mod Utility{

    use std::ffi;
    use std::ffi::CString;
    
    pub unsafe fn getFunction<T : Sized>(
        funcName:&str
        ,dll_handle:*mut winapi::shared::minwindef::HINSTANCE__
        ) -> Result<T,&str>
    {
        use winapi::um::libloaderapi::GetProcAddress;

        let size = std::mem::size_of::<T>();
        if size != 8{
            return Err("T size error");
        }
    
        let funcName=CString::new(funcName).expect("CString::new Fail!");
    
        // DLL에서 함수 포인터 얻기
        let pFunc = unsafe {GetProcAddress(dll_handle, funcName.as_ptr())};
        
        if pFunc.is_null() {
            println!("함수 '{}' 찾을 수 없음", funcName.to_str().unwrap());
            panic!("Function '{}' not founded", funcName.to_str().unwrap());
            //return Err("Function not founded");
        }
    
        // 함수 시그니처에 맞게 타입 캐스팅
        let func: T = unsafe 
        {
            std::mem::transmute_copy(&pFunc)
        };
        return Ok(func);
    }
    
    fn str_to_CString(str:&str) -> CString
    {
        return ffi::CString::new(str.to_string()).expect("CString::new failed");
    }
    
// macro_rules! cstr_to_ptr {
//     ($cstr:expr) => {{
//         $cstr.as_ptr() as *const ffi::c_char
//     }};
// }
}
