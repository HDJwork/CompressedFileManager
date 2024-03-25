#![allow(non_snake_case)]
#![allow(dead_code)]

mod CompressedFileManager;
mod CompressedFile;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let compressedFile=CompressedFileManager::Open("testPath");
        CompressedFileManager::Close(compressedFile);
        // let result = add(2, 2);
        // assert_eq!(result, 4);
    }
}
