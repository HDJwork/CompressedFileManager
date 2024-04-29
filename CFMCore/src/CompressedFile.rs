#![allow(non_snake_case)]

use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;

use crate::CompressManager::ICompressManager::ICompressManager;
use crate::PreviewedFile::IPreviewedFile::IPreviewedFile;
use std::collections::HashSet;

//T.B.D Dummy code
pub struct CompressedFile
{
    path : String,
    id : usize,

    manager : Box<dyn ICompressManager>,
    fileList : Vec<String>,
    deleteFileList: HashSet<String>,
}

impl CompressedFile{
    
    pub fn new(path:&str)-> CompressedFile
    {
        use crate::CompressManager::CompressManagerImpl::CompressManagerImpl;
        let mut retval = CompressedFile{
            path: String::from(path),
            id: Self::getID(),
            manager: Box::new(CompressManagerImpl::new(path)),
            fileList : Vec::new(),
            deleteFileList : HashSet::new(),
        };
        if retval.manager.Open(){
            retval.fileList = retval.manager.GetFileList();
        }
        return retval;
    }

    pub fn GetFileList(&mut self)->Vec<String>
    {
        if self.fileList.is_empty(){
            if !self.manager.IsOpen(){
                if !self.manager.Open(){
                    println!("[CompressedFile]Open Fail!");
                    return Vec::new();
                }
            }
        }
        return self.fileList.clone();
    }
    pub fn Recompress(&mut self, path:&str)->bool
    {
        return self.manager.Compress(path,Box::new(self.deleteFileList.clone().into_iter()));
    }
    pub fn DeleteFile(&mut self, file:&str)->bool
    {
        return self.deleteFileList.insert(String::from(file));
    }
    pub fn RevertDeletedFile(&mut self, file:&str)->bool
    {
        let strFile =String::from(file);
        return self.deleteFileList.remove(&strFile);
    }
    pub fn IsChanged(&self)->bool
    {
        return !self.deleteFileList.is_empty()
    }
    pub fn PreviewFile(&mut self)->Box<dyn IPreviewedFile>
    {
        //T.B.D
        #[cfg(debug_assertions)]
        return Box::new(crate::PreviewedFile::IPreviewedFile::DummyPreviewedFile{});
    }

    #[cfg(debug_assertions)]
    pub fn Summarize(&self) -> String
    {
        return String::from(self.id.to_string()+"("+ &self.path +")");
    }
    #[cfg(debug_assertions)]
    fn getID() -> usize
    {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        COUNTER.store(COUNTER.load(Ordering::Relaxed)+1,Ordering::Relaxed);
        return COUNTER.load(Ordering::Relaxed);
    }
}
impl Drop for CompressedFile{
    fn drop(&mut self)
    {
        self.manager.Close();
    }
}
