#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::ffi;
use std::ffi::CString;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress, LoadLibraryW, FreeLibrary};

// test C DLL Dynamic Link from winapi
// this code is supported chatgpt

unsafe fn getFunction<T>(dll_handle:*mut winapi::shared::minwindef::HINSTANCE__
    ,funcName:&str)->Result<T,&str>
{

    let funcName=CString::new(funcName).expect("CString::new Fail!");

    // DLL에서 함수 포인터 얻기
    let pFunc = unsafe {GetProcAddress(dll_handle, funcName.as_ptr())};
    
    if pFunc.is_null() {
        println!("함수 '{}' 찾을 수 없음", funcName.to_str().unwrap());
        return Err("");
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

macro_rules! cstr_to_ptr {
    ($cstr:expr) => {{
        $cstr.as_ptr() as *const ffi::c_char
    }};
}

fn main() {
    // DLL 파일 경로
    let dll_path = "../../../ref/miniz_DLL.dll";
    
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
        return;
    }

    //------------------------------------- DLL Function Load --------------------------------------

    let funcName="MINIZ_LIB_Read";
    type FnType_MINIZ_LIB_Read = unsafe extern "stdcall" fn(result: * mut ffi::c_ulonglong, filename:* const ffi::c_char)-> ffi::c_int;
    let fn_MINIZ_LIB_Read : FnType_MINIZ_LIB_Read=unsafe {getFunction(dll_handle,funcName).unwrap()};
    
    let funcName="MINIZ_LIB_Read_Result_Release";
    type FnType_MINIZ_LIB_Read_Result_Release = unsafe extern "stdcall" fn(result: * mut ffi::c_ulonglong)-> ffi::c_int;
    let fn_MINIZ_LIB_Read_Result_Release : FnType_MINIZ_LIB_Read_Result_Release=unsafe {getFunction(dll_handle,funcName).unwrap()};
    
    let funcName="MINIZ_LIB_Read_Result_GetErrorCode";
    type FnType_MINIZ_LIB_Read_Result_GetErrorCode = unsafe extern "stdcall" fn(result: * mut ffi::c_ulonglong)-> ffi::c_int;
    let fn_MINIZ_LIB_Read_Result_GetErrorCode: FnType_MINIZ_LIB_Read_Result_GetErrorCode = unsafe {getFunction(dll_handle,funcName).unwrap()};

    //------------------------------------- Call --------------------------------------
    //set parameter
    let mut ptr : ffi::c_ulonglong=0;
    //let path = "D:/Develop/CompressedFileManager/testproj/TestData/1.zip";
    let path = "TestData/1.zip";
    println!("path : {}",path);

    // 함수 호출
    unsafe {
        let readResult: *mut ffi::c_ulonglong = &mut ptr;

        println!("readresult = {}",ptr as u64);
        println!("STEP : fn_MINIZ_LIB_Read");
        let cstr = str_to_CString(path);
        let retval=fn_MINIZ_LIB_Read(readResult, cstr_to_ptr!(cstr));
        if retval != 0{
            println!("readresult = {}",ptr as u64);
        }
        else{

            println!("read fail!");
            println!("STEP : fn_MINIZ_LIB_Read_Result_GetErrorCode");
            println!("errorcode : {}", fn_MINIZ_LIB_Read_Result_GetErrorCode(readResult));
            println!("readresult = {}",ptr as u64);
        }
        println!("STEP : fn_MINIZ_LIB_Read_Result_Release");
        fn_MINIZ_LIB_Read_Result_Release(readResult);
        println!("readresult = {}",ptr as u64);
    }

    unsafe{
        if FreeLibrary(dll_handle) == 0
        {
            println!("DLL unload fail!");
        }
    }
}

// test C DLL Link from ffi
//use std::ffi;
//
//#[link(name="miniz_DLL", kind = "static")]
//extern "C"{
//    fn MINIZ_LIB_Read(result:* mut ffi::c_ulonglong, filename:&ffi::CStr)-> ffi::c_int;
//    fn MINIZ_LIB_Read_Result_Release(result:* mut ffi::c_ulonglong);
//    fn MINIZ_LIB_Read_Result_GetErrorCode(result:* mut ffi::c_ulonglong)-> ffi::c_int;
//}
// fn main() {
//     CompressedFileManager::Open("111");
//     let path:ffi::CString = ffi::CString::new("D:/Develop/RustStudy/testproj/TestData/1.zip".to_string()).expect("CString::new failed");
//     println!("path : {}",path.to_str().expect("FAIL!"));
//
//     unsafe{
//         let mut ptr : ffi::c_ulonglong=0;
//         let mut readResult: *mut ffi::c_ulonglong = &mut ptr;
//         println!("readresult = {}",ptr as u64);
//            
//         println!("MINIZ_LIB_Read");
//         let result=MINIZ_LIB_Read(readResult, path.as_c_str());
//         println!("result = {}",result);
//         println!("readresult = {}",ptr as u64);
//         if result != 0
//         {
//            
//         }
//         else
//         {
//             println!("read fail!");
//             println!("MINIZ_LIB_Read_Result_GetErrorCode");
//             println!("errorcode : {}", MINIZ_LIB_Read_Result_GetErrorCode(readResult));
//             println!("readresult = {}",ptr as u64);
//
//         }
//         println!("MINIZ_LIB_Read_Result_Release");
//         MINIZ_LIB_Read_Result_Release(readResult);
//         println!("readresult = {}",ptr as u64);
//     }
// }

// test module and function
//mod CompressedFileManager;
// fn main() {
//     CompressedFileManager::Open("111");
// }