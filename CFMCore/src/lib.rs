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

        let compressedFile=CompressedFileManager::Open("testPath");
        println!("compressedFile.GetFileList => \r\n{}",ToSummary(compressedFile.GetFileList()));
        
        CompressedFileManager::Close(compressedFile);
        let compressedFile=CompressedFileManager::Open("testPath");
        CompressedFileManager::Close(compressedFile);
        // let result = add(2, 2);
        // assert_eq!(result, 4);
        SingletonManager::cleanup();
    }
    fn ToSummary(strs:Vec<String>)->String
    {
        let mut retval=String::new();
        
        for (idx,element) in strs.iter().enumerate(){
            if idx!=0{ retval.push('\n');}
            retval.push_str(element.as_str());
            
        }
        
        return retval;
    }
}
