#![allow(non_snake_case)]
#![allow(dead_code)]

mod compressed_file_manager;
mod compressed_file;
mod compress_manager;
mod previewed_file;
mod custom_type;
mod singleton_manager;




#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn it_works() {
        //arrange
        use std::collections::HashSet;
        let srcPath="../TestData/TestData.zip";
        let testPath="../TestData/TestData1.zip";
        let exist = match std::fs::metadata(testPath){
            Ok(metadata)=>{metadata.is_file()}
            Err(_) => {false}
        };
        if exist {
            std::fs::remove_file(testPath).expect("std::fs::remove_file Fail!");
        }
        std::fs::copy(srcPath,testPath).expect("std::fs::copy Fail!");

        singleton_manager::startup();
        //let mut compressedFile=CompressedFileManager::Open("D:/Develop/CompressedFileManager/TestData/TestData.zip");

        //act
        println!("Open : {}",testPath);
        let mut compressedFile=compressed_file_manager::Open(testPath);
        let fileList=compressedFile.GetFileList();
        println!("compressedFile.GetFileList => \r\n{}",ToSummary(&fileList));
        let mut removeList: HashSet<String> = HashSet::new();
        removeList.insert(fileList[0].clone());

        println!("delete file =>");
        for removeFile in &removeList{
            println!("{}",removeFile.as_str());
            compressedFile.DeleteFile(removeFile.as_str());
        }
        compressedFile.Recompress(testPath);

        compressed_file_manager::Close(compressedFile);

        
        println!("Open : {}",testPath);
        let mut compressedFile=compressed_file_manager::Open(testPath);
        let fileList_recomp=compressedFile.GetFileList();
        println!("compressedFile.GetFileList => \r\n{}",ToSummary(&fileList_recomp));
        compressed_file_manager::Close(compressedFile);
        
        //assert
        let mut verificationList : Vec<String> = Vec::new();
        for file in &fileList{
            if removeList.contains(file) == false{
                verificationList.push(file.clone()); 
            }
        }
        let count = verificationList.len();
        assert_eq!(verificationList.len(),fileList_recomp.len());
        for i in 0..count{
            assert_eq!(verificationList[i],fileList_recomp[i]);
        }

        //cleanup
        singleton_manager::cleanup();
        
        let exist = match std::fs::metadata(testPath){
            Ok(metadata)=>{metadata.is_file()}
            Err(_) => {false}
        };
        if exist {
            std::fs::remove_file(testPath).expect("std::fs::remove_file Fail!");
        }
    }
    fn ToSummary(strs:&Vec<String>)->String
    {
        let mut retval=String::new();
        
        for (idx,element) in strs.iter().enumerate(){
            if idx!=0{ retval.push('\n');}
            retval.push_str(element.as_str());
            
        }
        
        return retval;
    }
}
