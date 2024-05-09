#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

pub mod Type_C{

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

    pub const C_TRUE : C_BOOL = 1;
    pub const C_FALSE : C_BOOL = 0;
    pub const C_HANDLE_NULL : C_HANDLE = 0;
    pub const C_BUFFER_MAX : C_INT = 200;
}

pub mod Utility_C{
    use super::Type_C::*;
    use std::ffi::CString;

    pub fn handle_to_ptr(handle:& mut C_HANDLE) -> C_PTR
    {
        let readResult: *mut std::ffi::c_ulonglong = handle as *mut C_HANDLE;
        return readResult;
    }

    pub fn ptr_to_ref<T>(ptr: C_PTR) -> &'static mut T
    {
        return unsafe{
            let _ptrValue = *ptr as u64;
            let raw_ptr:*mut u8=_ptrValue as *mut u8;
            let targetPtr = raw_ptr as *mut T;
            targetPtr.as_mut().unwrap()
        };
    }
    pub fn ptr_to_ptr<T>(ptr: C_PTR) -> *mut T
    {
        return unsafe{
            let _ptrValue = *ptr as u64;
            let raw_ptr:*mut u8=_ptrValue as *mut u8;
            let targetPtr = raw_ptr as *mut T;
            targetPtr
        };
    }

    #[allow(unused_macros)]
    #[macro_export]
    macro_rules! cstr_to_ptr {
        ($cstr:expr) => {{
            $cstr.as_ptr() as *const std::ffi::c_char
        }};
    }

    pub fn str_to_CString(str:&str) -> C_CSTRING
    {
        return std::ffi::CString::new(str.to_string()).expect("CString::new failed");
    }

    pub fn string_Write_to_CBuffer(input_string:String, buffer : C_STR, buffer_count : C_INT) -> bool
    {
        // From Chat GPT
        // Rust 문자열을 C 스타일의 문자열로 변환
        let c_string = CString::new(input_string).expect("Failed to convert String to CString.");

        // C 문자열로 변환된 Rust 문자열의 길이를 가져옴 (null 문자 포함)
        let c_string_length = c_string.as_bytes_with_nul().len();

        // 버퍼의 크기가 충분한지 확인
        if buffer_count < c_string_length as C_INT {
            return false;
        }

        // C 문자열로 변환된 Rust 문자열을 주어진 버퍼에 복사
        unsafe {
            // C 문자열을 복사하는 함수 호출
            libc::strncpy(buffer as *mut i8, c_string.as_ptr(),buffer_count as usize);
        }

        return true;
    }

    pub fn ptr_to_String(buff :C_STR) -> String
    {
        let cstr = unsafe{std::ffi::CStr::from_ptr(buff)};
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

    pub fn vec_to_String(buff :&Vec<C_CHAR>) -> String
    {
        return ptr_to_String(buff.as_ptr());
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