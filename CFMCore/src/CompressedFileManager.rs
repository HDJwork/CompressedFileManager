#![allow(non_snake_case)]
//CompressedFileManager.rs

use crate::CompressedFile::CompressedFile;


//T.B.D need to test object create in dll
pub fn Open(path :&str)->Box<CompressedFile>
{
    let retval : Box<CompressedFile>=CompressedFile::Create(path);
    
    #[cfg(debug_assertions)]
    dbg!(format!("CompressedFileManager::Open => {}",retval.Summarize()));
    
    return retval;
}
pub fn Close(compressedFile :Box<CompressedFile>)
{
    //do nothing
    #[cfg(debug_assertions)]
    dbg!(format!("CompressedFileManager::Close => {}",compressedFile.Summarize()));
}