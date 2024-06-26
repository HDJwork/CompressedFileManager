#![allow(non_snake_case)]

#[cfg(debug_assertions)]
use core::sync::atomic::AtomicUsize;
#[cfg(debug_assertions)]
use core::sync::atomic::Ordering;


use super::compress_manager::ICompressManager;
use super::previewed_file::IPreviewedFile;
use std::collections::HashSet;

pub struct CompressedFile
{
    path : String,
    
    manager : Box<dyn ICompressManager>,
    fileList : Vec<String>,
    deleteFileList: HashSet<String>,
    #[cfg(debug_assertions)]
    id : usize,
}

impl CompressedFile{
    
    pub fn new(path:&str)-> CompressedFile    {
        use super::compress_manager::compress_manager_impl::CompressManagerImpl;
        let mut retval = CompressedFile{
            path: String::from(path),
            manager: Box::new(CompressManagerImpl::new(path)),
            fileList : Vec::new(),
            deleteFileList : HashSet::new(),
            #[cfg(debug_assertions)]
            id: Self::getID(),
        };
        if retval.manager.Open(){
            retval.fileList = retval.manager.GetFileList();
        }
        return retval;
    }
    pub fn new_without_open(path:&str)-> CompressedFile    {
        use super::compress_manager::compress_manager_impl::CompressManagerImpl;
        let retval = CompressedFile{
            path: String::from(path),
            manager: Box::new(CompressManagerImpl::new(path)),
            fileList : Vec::new(),
            deleteFileList : HashSet::new(),
            #[cfg(debug_assertions)]
            id: Self::getID(),
        };
        // if retval.manager.Open(){
        //     retval.fileList = retval.manager.GetFileList();
        // }
        return retval;
    }
    pub fn Open(&mut self)->bool    {
        match self.manager.Open(){
            true=>{
                self.fileList = self.manager.GetFileList();
                return true;
            },
            false=>{
                return false;
            }
        }
    }

    pub fn GetFileList(&mut self)->Vec<String>    {
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
    
    pub fn Recompress(&mut self, path:&str)->bool    {
        return self.manager.Compress(path,Box::new(self.deleteFileList.clone().into_iter()));
    }
    
    pub fn DeleteFile(&mut self, file:&str)->bool    {
        return self.deleteFileList.insert(String::from(file));
    }
    
    pub fn RevertDeletedFile(&mut self, file:&str)->bool    {
        let strFile =String::from(file);
        return self.deleteFileList.remove(&strFile);
    }
    
    pub fn IsChanged(&self)->bool    {
        return !self.deleteFileList.is_empty()
    }
    
    pub fn PreviewFile(&mut self,file:&str)->Result<&Box<dyn IPreviewedFile>,String>    {
        return self.manager.PreviewFile(file);
    }

    pub fn ReleaseObject(&mut self){
        //dbg!("CompressedFile.ReleaseObject!");
        self.manager.Close();

    }

    #[cfg(debug_assertions)]
    pub fn Summarize(&self) -> String    {
        return String::from(self.id.to_string()+"("+ &self.path +")");
    }
    
    #[cfg(debug_assertions)]
    fn getID() -> usize    {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        COUNTER.store(COUNTER.load(Ordering::Relaxed)+1,Ordering::Relaxed);
        return COUNTER.load(Ordering::Relaxed);
    }
}
impl Drop for CompressedFile{
    fn drop(&mut self)    {
        dbg!("CompressedFile.drop!");
        self.ReleaseObject();
    }
}
