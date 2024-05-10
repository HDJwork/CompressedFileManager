
use super::compress_manager::MinizWrapperDllObj;
use super::logger::logger::Logger;

static mut STARTUP_COUNT:i32=0;

pub fn startup()
{
    if unsafe{STARTUP_COUNT==0}{
        MinizWrapperDllObj::instance();
    }    
    unsafe{STARTUP_COUNT=STARTUP_COUNT+1;}
}

pub fn cleanup()
{
    unsafe{STARTUP_COUNT=STARTUP_COUNT-1;}

    if unsafe{STARTUP_COUNT==0}{
        MinizWrapperDllObj::destroy();
    }
    Logger::destroy();
}