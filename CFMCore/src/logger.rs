
#[cfg(not(feature = "use_logger"))]
pub mod logger{
    pub struct Logger{
    }
    
    impl Logger{
        //singleton
        pub fn instance() -> Logger {
            return Logger{};
        }

        pub fn destroy() {
        }

        pub fn write(&mut self,_ :String)
        {
        }
    }
}
#[cfg(feature = "use_logger")]
pub mod logger{
    use std::io::Write;
    pub struct Logger{
        path : String,
    }

    static mut INSTANCE : Option<&'static mut Logger> = None;

    impl Logger{
        //singleton
        pub fn instance() -> &'static mut Logger {
            unsafe {
                match INSTANCE {
                    Option::Some(ref mut manager) => *manager,
                    Option::None => {
                        println!("new instance!");
                        let logPath="D:/cfm_core_log.txt";
                        let manager_box = Box::new(Logger{path:String::from(logPath)});
                        let manager_raw = Box::into_raw(manager_box);
                        INSTANCE = Some(&mut *manager_raw);
                        let mut file = std::fs::File::create(logPath).unwrap();
                        let _= file.write_all("".as_bytes());
                        &mut *manager_raw
        }
                }
            }
        }

        pub fn destroy() {
            unsafe {
                #[allow(static_mut_refs)]
                if let Some(raw) = std::mem::replace(&mut INSTANCE, None) {
                    let _ = Box::from_raw(raw);
                }
            }
        }

        pub fn write(&mut self,text :String)
        {
            use std::fs::OpenOptions;
            //use std::io::Write;
            // 파일을 쓰기 전용으로 열기 (기존 파일이 있다면 덮어씁니다)
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(String::from(&self.path))
                .unwrap();

            // 파일에 내용 쓰기
            let _= file.write_all(text.as_bytes());


        }
    }

}