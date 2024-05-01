#![allow(non_snake_case)]
#![allow(dead_code)]

mod CompressedFileManager;
mod CompressedFile;
mod CompressManager;
mod PreviewedFile;
mod Type;
mod SingletonManager;


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn it_works() {
        SingletonManager::startup();

        //let mut compressedFile=CompressedFileManager::Open("D:/Develop/CompressedFileManager/TestData/TestData.zip");
        let path="../TestData/TestData.zip";
        let outputPath="../TestData/TestData1.zip";
        println!("Open : {}",path);
        let mut compressedFile=CompressedFileManager::Open(path);
        let fileList=compressedFile.GetFileList();
        println!("compressedFile.GetFileList => \r\n{}",ToSummary(&fileList));
        
        println!("delete file : {}",fileList[0].as_str());
        compressedFile.DeleteFile(fileList[0].as_str());
        compressedFile.Recompress(outputPath);

        CompressedFileManager::Close(compressedFile);

        
        println!("Open : {}",outputPath);
        let mut compressedFile=CompressedFileManager::Open(outputPath);
        let fileList=compressedFile.GetFileList();
        println!("compressedFile.GetFileList => \r\n{}",ToSummary(&fileList));
        CompressedFileManager::Close(compressedFile);
        
        // // let result = add(2, 2);
        // // assert_eq!(result, 4);

        SingletonManager::cleanup();
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
