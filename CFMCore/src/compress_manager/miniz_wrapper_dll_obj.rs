#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

//use std::sync::{Arc,Mutex};
//use std::cell::OnceCell;

//#[macro_use]
//extern crate lazy_static;
//use lazy_static::lazy_static;
//---------------------------------------------------------- Type Definition --------------------------------------------------------------------
//C Type
pub type C_HANDLE = std::ffi::c_ulonglong;
pub type C_PTR = * mut C_HANDLE;
pub type C_BOOL = std::ffi::c_int;
pub type C_INT = std::ffi::c_int;
pub type C_CHAR = std::ffi::c_char;
pub type C_STR = * const C_CHAR;
pub type C_STRS = * const C_STR;
pub type C_CSTRING = std::ffi::CString;

#[allow(dead_code)]
pub const C_TRUE : C_BOOL = 1;
pub const C_FALSE : C_BOOL = 0;
pub const C_HANDLE_NULL : C_HANDLE = 0;
pub const C_BUFFER_MAX : C_INT = 200;

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

pub mod Utility{
    use crate::compress_manager::miniz_wrapper_dll_obj::{
        C_HANDLE,
        C_PTR,
        C_CHAR,
        C_CSTRING,
    };

    pub fn handle_to_ptr(handle:& mut C_HANDLE) -> C_PTR
    {
        let readResult: *mut std::ffi::c_ulonglong = handle as *mut C_HANDLE;
        return readResult;
    }

    #[allow(unused_macros)]
    macro_rules! cstr_to_ptr {
        ($cstr:expr) => {{
            $cstr.as_ptr() as *const std::ffi::c_char
        }};
    }
    
    pub fn str_to_CString(str:&str) -> C_CSTRING
    {
        return std::ffi::CString::new(str.to_string()).expect("CString::new failed");
    }
    
    pub fn to_string(buff :&Vec<C_CHAR>) -> String
    {
        let cstr = unsafe{std::ffi::CStr::from_ptr(buff.as_ptr())};
        let str:String;
        if let Ok(s) = cstr.to_str() {
            use std::str::FromStr;
            str=String::from_str(s).expect("String::from_str");
        } else {
            println!("UTF-8로 변환할 수 없는 문자열입니다.");
            // 대체 문자열을 얻기 위해 to_string_lossy() 메서드 사용
            let tmp=cstr.to_string_lossy().into_owned();
            println!("{}",tmp);
            str=tmp;
        }
        return str;
    }
    
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
    
        let funcName=std::ffi::CString::new(funcName).expect("CString::new Fail!");
    
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
    
}
