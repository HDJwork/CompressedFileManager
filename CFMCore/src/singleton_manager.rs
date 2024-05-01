
use super::compress_manager::MinizWrapperDllObj;


pub fn startup()
{
    MinizWrapperDllObj::instance();
}

pub fn cleanup()
{
    MinizWrapperDllObj::destroy();
}