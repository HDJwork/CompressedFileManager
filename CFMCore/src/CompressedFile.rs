#![allow(non_snake_case)]

use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;

use crate::CompressManager::ICompressManager::ICompressManager;
use crate::PreviewedFile::IPreviewedFile::IPreviewedFile;

//T.B.D Dummy code
pub struct CompressedFile
{
    path : String,
    id : usize,

    manager : Box<dyn ICompressManager>,
}

impl CompressedFile{
    
    pub fn new(path:&str)-> CompressedFile
    {
        use crate::CompressManager::CompressManagerImpl::CompressManagerImpl;
        //T.B.D Test code
        #[cfg(debug_assertions)]
        return CompressedFile{
            path: String::from(path),
            id: Self::getID(),
            manager: Box::new(CompressManagerImpl::new()),
        };
    }

    pub fn GetFileList(&self)->Vec<String>
    {
        return self.manager.GetFileList();
    }
    pub fn Recompress(&self, path:&str)->bool
    {
        //T.B.D
        return self.manager.Compress();
    }
    pub fn DeleteFile(&self, file:&str)->bool
    {
        //T.B.D
        return false;
    }
    pub fn IsChanged(&self)->bool
    {
        //T.B.D
        return false;
    }
    pub fn PreviewFile(&self)->Box<dyn IPreviewedFile>
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
