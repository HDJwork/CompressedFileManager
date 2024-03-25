#![allow(non_snake_case)]

use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering;

//T.B.D Dummy code
pub struct CompressedFile
{
    path : String,
    id : usize,
}

impl CompressedFile{
    pub fn Summarize(&self) -> String
    {
        return String::from(self.id.to_string()+"("+ &self.path +")");
    }
}

pub fn CreateCompressedFile(path:&str)-> Box<CompressedFile>
{
    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    return Box::new(CompressedFile{
        path: String::from(path),
        id: COUNTER.load(Ordering::Relaxed),
    });
}