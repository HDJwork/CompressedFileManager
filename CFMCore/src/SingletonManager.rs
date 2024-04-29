use crate::CompressManager::MinizWrapperDllObj::MinizWrapperDllObj;


pub fn startup()
{
    MinizWrapperDllObj::instance();
}

pub fn cleanup()
{
    MinizWrapperDllObj::destroy();
}