#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::{ffi, str::FromStr};
use std::ffi::CString;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use winapi::um::libloaderapi::{GetModuleHandleW, GetProcAddress, LoadLibraryW, FreeLibrary};

// test C DLL Dynamic Link from winapi
// this code is supported chatgpt

type C_PTR = * mut ffi::c_ulonglong;
type C_BOOL = ffi::c_int;
const C_FALSE : C_BOOL = 0;
#[allow(dead_code)]
const C_TRUE : C_BOOL = 1;

unsafe fn getFunction<T : Sized>(dll_handle:*mut winapi::shared::minwindef::HINSTANCE__
    ,funcName:&str)->Result<T,&str>
{
    let size = std::mem::size_of::<T>();
    if size != 8{
        return Err("T size error");
    }

    let funcName=CString::new(funcName).expect("CString::new Fail!");

    // DLL에서 함수 포인터 얻기
    let pFunc = unsafe {GetProcAddress(dll_handle, funcName.as_ptr())};
    
    if pFunc.is_null() {
        println!("함수 '{}' 찾을 수 없음", funcName.to_str().unwrap());
        return Err("Function not founded");
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
trait Foo {}

struct Bar {}

impl Foo for Bar {}

impl Drop for Bar {
    fn drop(&mut self) {
        println!("dropped")
    }
}

fn main() {
    //let x = Box::new( Bar {} ) as Box<dyn Foo>;
    let _y = Option::Some(Box::new( Bar {} ));
}
// fn main() {

    
//     // DLL 파일 경로
//     let dll_path = "../../../ref/miniz_DLL.dll";
    
//     // DLL 파일 경로를 Wide 문자열로 변환
//     let dll_path_wide: Vec<u16> = OsStr::new(dll_path)
//         .encode_wide()
//         .chain(std::iter::once(0))
//         .collect();

//     //------------------------------------- DLL Load --------------------------------------
//     // 
//     let dll_handle = unsafe { LoadLibraryW(dll_path_wide.as_ptr()) };
//     if dll_handle.is_null() {
//         println!("DLL Load Fail!");
//         return;
//     }

//     //------------------------------------- DLL Function Load --------------------------------------

//     let funcName="MINIZ_LIB_Read";
//     type FnType_MINIZ_LIB_Read = unsafe extern "stdcall" fn(result: C_PTR, filename:* const ffi::c_char)-> C_BOOL;
//     let fn_MINIZ_LIB_Read : FnType_MINIZ_LIB_Read=unsafe {getFunction(dll_handle,funcName).unwrap()};
    
//     let funcName="MINIZ_LIB_Read_Result_Release";
//     type FnType_MINIZ_LIB_Read_Result_Release = unsafe extern "stdcall" fn(result: C_PTR)-> C_BOOL;
//     let fn_MINIZ_LIB_Read_Result_Release : FnType_MINIZ_LIB_Read_Result_Release=unsafe {getFunction(dll_handle,funcName).unwrap()};
    
//     let funcName="MINIZ_LIB_Read_Result_GetErrorCode";
//     type FnType_MINIZ_LIB_Read_Result_GetErrorCode = unsafe extern "stdcall" fn(result: C_PTR)-> ffi::c_int;
//     let fn_MINIZ_LIB_Read_Result_GetErrorCode: FnType_MINIZ_LIB_Read_Result_GetErrorCode = unsafe {getFunction(dll_handle,funcName).unwrap()};
    
//     let funcName="MINIZ_LIB_Read_Result_GetCount";
//     type FnType_MINIZ_LIB_Read_Result_GetCount = unsafe extern "stdcall" fn(result: C_PTR) -> ffi::c_int;
//     let fn_MINIZ_LIB_Read_Result_GetCount: FnType_MINIZ_LIB_Read_Result_GetCount = unsafe {getFunction(dll_handle,funcName).unwrap()};

//     //let funcName="MINIZ_LIB_Read_Result_GetFileName";
//     //type FnType_MINIZ_LIB_Read_Result_GetFileName = unsafe extern "stdcall" fn(result: C_PTR, index: ffi::c_int, buff:* const ffi::c_char, buffCount: ffi::c_int) -> C_BOOL;
//     //let fn_MINIZ_LIB_Read_Result_GetFileName: FnType_MINIZ_LIB_Read_Result_GetFileName = unsafe {getFunction(dll_handle,funcName).unwrap()};
    
//     let funcName="MINIZ_LIB_Read_Result_GetFileName_UTF8";
//     type FnType_MINIZ_LIB_Read_Result_GetFileName_UTF8 = unsafe extern "stdcall" fn(result: C_PTR, index: ffi::c_int, buff:* const ffi::c_char, buffCount: ffi::c_int) -> C_BOOL;
//     let fn_MINIZ_LIB_Read_Result_GetFileName_UTF8: FnType_MINIZ_LIB_Read_Result_GetFileName_UTF8 = unsafe {getFunction(dll_handle,funcName).unwrap()};
    

//     //------------------------------------- Call --------------------------------------
//     //set parameter
//     let mut ptr : ffi::c_ulonglong=0;
//     //let path = "D:/Develop/CompressedFileManager/testproj/TestData/1.zip";
//     //let path = "TestData/1.zip";
//     let path = "TestData/TestData.zip";
    
//     println!("path : {}",path);

//     // 함수 호출
//     unsafe {
//         let readResult: *mut ffi::c_ulonglong = &mut ptr;

//         println!("readresult = {}",ptr as u64);
//         println!("STEP : fn_MINIZ_LIB_Read");
//         let cstr = str_to_CString(path);
//         let retval=fn_MINIZ_LIB_Read(readResult, cstr_to_ptr!(cstr));
        
//         if retval != C_FALSE {
//             println!("readresult = {}",ptr as u64);

//             let mut fileNameList : Vec<String> = Vec::new();

//             println!("STEP : fn_MINIZ_GetCount");
//             let count = fn_MINIZ_LIB_Read_Result_GetCount(readResult);
//             println!("Count : {}",count);
//             println!("STEP : fn_MINIZ_LIB_Read_Result_GetFileName");
//             for i in 0..count{
//                 let mut buff : Vec<ffi::c_char> = Vec::new();
//                 buff.resize(200, 0);
//                 let retval=fn_MINIZ_LIB_Read_Result_GetFileName_UTF8(readResult,i,buff.as_ptr() as * const ffi::c_char,200);
//                 if retval != C_FALSE
//                 {
//                     // Vec<ffi::c_char> to String
//                     let cstr = ffi::CStr::from_ptr(buff.as_ptr());
//                     let str:String;
//                     if let Ok(s) = cstr.to_str() {
//                         str=String::from_str(s).expect("String::from_str");
//                     } else {
//                         println!("UTF-8로 변환할 수 없는 문자열입니다.");
//                         // 대체 문자열을 얻기 위해 to_string_lossy() 메서드 사용
//                         let tmp=cstr.to_string_lossy().into_owned();
//                         println!("{}",tmp);
//                         str=tmp;
//                     }
//                     fileNameList.push(str);
//                 }
//             }
//             println!("STEP : Show file list");
//             println!("File List : ");
//             for fileName in fileNameList{
//                 println!("{}",fileName);
//             }
            
//         }
//         else{

//             println!("read fail!");
//             println!("STEP : fn_MINIZ_LIB_Read_Result_GetErrorCode");
//             println!("errorcode : {}", fn_MINIZ_LIB_Read_Result_GetErrorCode(readResult));
//             println!("readresult = {}",ptr as u64);
//         }
//         println!("STEP : fn_MINIZ_LIB_Read_Result_Release");
//         let retval = fn_MINIZ_LIB_Read_Result_Release(readResult);
//         if retval == C_FALSE{
//             println!(" fn_MINIZ_LIB_Read_Result_Release is Fail!");
//         }
//         println!("readresult = {}",ptr as u64);
//     }

//     unsafe{
//         if FreeLibrary(dll_handle) == 0
//         {
//             println!("DLL unload fail!");
//         }
//     }
// }

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