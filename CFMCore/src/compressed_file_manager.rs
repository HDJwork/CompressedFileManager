#![allow(non_snake_case)]
#![allow(non_camel_case_types)]


#[cfg(test)]
pub mod TestApi{
    //use super::super::*;
    use super::super::compressed_file::CompressedFile;
//T.B.D need to test object create in dll
    pub fn Open(path :&str)->Box<CompressedFile>
    {
        let retval : Box<CompressedFile>=Box::new(CompressedFile::new(path));
        
        #[cfg(debug_assertions)]
        {
            let text=format!("CompressedFileManager::Open => {}",retval.Summarize());
            dbg!(text);
        }
        
        return retval;
    }
    pub fn Close(_compressedFile :Box<CompressedFile>)
    {
        //do nothing
        #[cfg(debug_assertions)]
        {
            let text=format!("CompressedFileManager::Close => {}",_compressedFile.Summarize());
            dbg!(text);
        }
        
    }

}