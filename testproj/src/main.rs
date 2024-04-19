#[allow(non_snake_case)]

use std::ffi;

mod CompressedFileManager;
#[link(name="miniz_DLL", kind = "static")]
extern "C"{
    fn MINIZ_LIB_Read(result:* mut ffi::c_ulonglong, filename:&ffi::CStr)-> ffi::c_int;
    fn MINIZ_LIB_Read_Result_Release(result:* mut ffi::c_ulonglong);
    fn MINIZ_LIB_Read_Result_GetErrorCode(result:* mut ffi::c_ulonglong)-> ffi::c_int;
}

fn main() {
    CompressedFileManager::Open("111");
    let path:ffi::CString = ffi::CString::new("D:/Develop/RustStudy/testproj/TestData/1.zip".to_string()).expect("CString::new failed");
    println!("path : {}",path.to_str().expect("FAIL!"));

    unsafe{
        let mut ptr : ffi::c_ulonglong=0;
        let mut readResult: *mut ffi::c_ulonglong = &mut ptr;
        println!("readresult = {}",ptr as u64);
            
        println!("MINIZ_LIB_Read");
        let result=MINIZ_LIB_Read(readResult, path.as_c_str());
        println!("result = {}",result);
        println!("readresult = {}",ptr as u64);
        if result != 0
        {
            
        }
        else
        {
            println!("read fail!");
            println!("MINIZ_LIB_Read_Result_GetErrorCode");
            println!("errorcode : {}", MINIZ_LIB_Read_Result_GetErrorCode(readResult));
            println!("readresult = {}",ptr as u64);

        }
        println!("MINIZ_LIB_Read_Result_Release");
        MINIZ_LIB_Read_Result_Release(readResult);
        println!("readresult = {}",ptr as u64);
    }
}
