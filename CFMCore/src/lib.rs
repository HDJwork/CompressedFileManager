#![allow(non_snake_case)]
#![allow(dead_code)]

mod compressed_file_manager;
mod compressed_file;
mod compress_manager;
mod previewed_file;
mod custom_type;
mod singleton_manager;
mod utility_c;
mod api_dll;



#[cfg(test)]
mod tests {


    use super::*;
    fn to_summary(strs:&Vec<String>)->String    {
        let mut retval=String::new();
        
        for (idx,element) in strs.iter().enumerate(){
            if idx!=0{ retval.push('\n');}
            retval.push_str(element.as_str());
            
        }
        
        return retval;
    }


    #[test]
    fn test_recompress() {
        use compressed_file_manager::TestApi;
        //---------------------------------- arrange ---------------------------------- 
        singleton_manager::startup();
        use std::collections::HashSet;
        let srcPath="../TestData/TestData.zip";
        //make clone
        let testPath="../TestData/TestData1.zip";
        let exist = match std::fs::metadata(testPath){
            Ok(metadata)=>{metadata.is_file()}
            Err(_) => {false}
        };
        if exist {
            std::fs::remove_file(testPath).expect("std::fs::remove_file Fail!");
        }
        std::fs::copy(srcPath,testPath).expect("std::fs::copy Fail!");

        //let mut compressedFile=CompressedFileManager::Open("D:/Develop/CompressedFileManager/TestData/TestData.zip");

        //---------------------------------- act ----------------------------------
        //open
        println!("Open : {}",testPath);
        let mut compressedFile=TestApi::Open(testPath);
        let fileList=compressedFile.GetFileList();
        println!("compressedFile.GetFileList => \r\n{}",to_summary(&fileList));

        //set delete file list
        let mut removeList: HashSet<String> = HashSet::new();
        removeList.insert(fileList[0].clone());
        //delete file
        println!("delete file =>");
        for removeFile in &removeList{
            println!("{}",removeFile.as_str());
            compressedFile.DeleteFile(removeFile.as_str());
        }

        //recompress
        compressedFile.Recompress(testPath);

        TestApi::Close(compressedFile);

        
        
        //---------------------------------- assert ---------------------------------- 
        //open recompress file
        println!("Open : {}",testPath);
        let mut compressedFile=TestApi::Open(testPath);
        let fileList_recomp=compressedFile.GetFileList();
        println!("compressedFile.GetFileList => \r\n{}",to_summary(&fileList_recomp));
        TestApi::Close(compressedFile);

        //set verificationList
        let mut verificationList : Vec<String> = Vec::new();
        for file in &fileList{
            if removeList.contains(file) == false{
                verificationList.push(file.clone()); 
            }
        }
        //assert
        let count = verificationList.len();
        assert_eq!(verificationList.len(),fileList_recomp.len());
        for i in 0..count{
            assert_eq!(verificationList[i],fileList_recomp[i]);
        }

        //cleanup
        //remove test file
        let exist = match std::fs::metadata(testPath){
            Ok(metadata)=>{metadata.is_file()}
            Err(_) => {false}
        };
        if exist {
            std::fs::remove_file(testPath).expect("std::fs::remove_file Fail!");
        }

        singleton_manager::cleanup();
    }

    #[test]
    fn test_preview() {
        use compressed_file_manager::TestApi;
        //---------------------------------- arrange ---------------------------------- 
        singleton_manager::startup();
        let srcPath="../TestData/TestData.zip";
        //let mut compressedFile=CompressedFileManager::Open("D:/Develop/CompressedFileManager/TestData/TestData.zip");

        //---------------------------------- act ---------------------------------- 
        println!("Open : {}",srcPath);
        let mut compressedFile=TestApi::Open(srcPath);
        let fileList=compressedFile.GetFileList();
        println!("compressedFile.GetFileList => \r\n{}",to_summary(&fileList));
        let previewTarget = fileList[0].clone();
        let result = compressedFile.PreviewFile(previewTarget.as_str());
        
        //---------------------------------- assert ---------------------------------- 
        //check result
        match result {
            Ok(previewedFile)=>{
                let tmpPath=previewedFile.GetTmpPath();
                println!("GetTmpPath => {}",tmpPath.as_str());
                //check temp file
                let exist = match std::fs::metadata(tmpPath.clone()){
                    Ok(metadata)=>{metadata.is_file()}
                    Err(_) => {false}
                };
                assert!(exist);
                
                //check temp remove
                TestApi::Close(compressedFile);
                let exist = match std::fs::metadata(tmpPath.clone()){
                    Ok(metadata)=>{metadata.is_file()}
                    Err(_) => {false}
                };
                assert!(!exist);
            }
            Err(e)=>{
                dbg!("{}",e.as_str());
                assert!(false);        
            }
        }

        //cleanup
        
        singleton_manager::cleanup();
    }

    #[test]
    fn test_Open_c() {
        use api_dll::*;
        use utility_c::Utility_C;
        use utility_c::Type_C::*;
        
        //---------------------------------- arrange ---------------------------------- 
        Startup();
        let srcPath="../TestData/TestData.zip";
        let testPath_CStr = Utility_C::str_to_CString(srcPath);
        let testPath_CPtr = cstr_to_ptr!(testPath_CStr);
        //let mut compressedFile=CompressedFileManager::Open("D:/Develop/CompressedFileManager/TestData/TestData.zip");

        //---------------------------------- act ----------------------------------
        //open
        let mut handle:C_HANDLE=C_HANDLE_NULL;
        let ptr=Utility_C::handle_to_ptr(&mut handle);
        println!("HANDLE = {}",handle);

        println!("1. Open => {}",srcPath);
        let openResult=Open(ptr,testPath_CPtr);
        println!("open result : {}",openResult);
        println!("HANDLE : {}",handle);

        let count = GetFileCount(ptr);
        println!("2. Count : {}",count);

        
        let mut buff : Vec<C_CHAR> = Vec::new();
        buff.resize(C_BUFFER_MAX as usize, 0);
        let getFileResult = GetFile(ptr,0,buff.as_ptr(),C_BUFFER_MAX);
        println!("3. GetFile Result : {}, FileList[0] : {}",getFileResult,Utility_C::vec_to_String(&buff));

        Close(ptr);
        println!("4. Close");
        println!("HANDLE = {}",handle);

        
        
        //---------------------------------- assert ---------------------------------- 
        //open recompress file
        

        Cleanup();
    }

}
