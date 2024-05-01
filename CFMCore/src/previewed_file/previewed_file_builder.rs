#![allow(non_snake_case)]

use super::*;

//from chatgpt
fn is_image_file(file_path: &str) -> std::io::Result<bool> {
    use std::io::Read;

    let mut file = std::fs::File::open(file_path)?;
    let mut buffer = [0; 12]; // 파일의 처음 12바이트를 읽습니다.

    file.read_exact(&mut buffer)?;

    // JPEG 파일의 매직 넘버는 0xFFD8FF
    let is_jpeg = buffer[0] == 0xFF && buffer[1] == 0xD8 && buffer[2] == 0xFF;

    // PNG 파일의 매직 넘버는 0x89504E47
    let is_png = buffer[0] == 0x89 && buffer[1] == 0x50 && buffer[2] == 0x4E && buffer[3] == 0x47;

    // GIF 파일의 매직 넘버는 "GIF87a" 또는 "GIF89a"
    let is_gif = &buffer[0..3] == b"GIF";

    // BMP 파일의 매직 넘버는 "BM"
    let is_bmp = &buffer[0..2] == b"BM";

    // TIFF 파일의 매직 넘버는 "II*" 또는 "MM*"
    let is_tiff = &buffer[0..2] == b"II" || &buffer[0..2] == b"MM";

    // WebP 파일의 매직 넘버는 "RIFF" + "WEBP"
    let is_webp = &buffer[0..4] == b"RIFF" && &buffer[8..12] == b"WEBP";

    Ok(is_jpeg || is_png || is_gif || is_bmp || is_tiff || is_webp)
}


pub fn buildPreviewedFile(file:&str)->Result<Box<dyn IPreviewedFile>,String>{

    let exist = match std::fs::metadata(file){
        Ok(metadata)=>{metadata.is_file()}
        Err(_) => {false}
    };
    if !exist {
        return Err(String::from("File not found!"));
    }

    if is_image_file(file).is_ok(){
        return Ok(Box::new(PreviewedFile_Image::new(file)));
    }
    else{
        return Ok(Box::new(PreviewedFile_Unknown::new(file)));
    }

}