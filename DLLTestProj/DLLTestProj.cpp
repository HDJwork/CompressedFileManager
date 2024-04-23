// DLLTestProj.cpp : 이 파일에는 'main' 함수가 포함됩니다. 거기서 프로그램 실행이 시작되고 종료됩니다.
//

#include <iostream>
#include"../miniz_DLL/API.h"

int main()
{
    std::cout << "Hello World!\n";
    PTR _ptr = 0;
    PTR *ptr = &_ptr;
    
    if (MINIZ_LIB_Read(ptr, "D:/Develop/CompressedFileManager/testproj/TestData/TestData.zip"))
    {
        std::cout << "ptr : " << ptr << std::endl;
        int count = MINIZ_LIB_Read_Result_GetCount(ptr);
        std::cout << "count : " << count << std::endl;
        std::cout << "file list : " << std::endl;
        for (int i = 0; i < count; ++i)
        {
            char buff[200];
            if (!MINIZ_LIB_Read_Result_GetFileName(ptr, i, buff, 200))
            {
                std::cout << "MINIZ_LIB_Read_Result_GetFileName Fail!" << ptr << std::endl;
                break;
            }
            std::cout << buff << std::endl;
        }

    }
    else
    {
        std::cout << "read fail!" << std::endl;
        std::cout << "error code : "<<MINIZ_LIB_Read_Result_GetErrorCode(ptr) << std::endl;

    }
    MINIZ_LIB_Read_Result_Release(ptr);
    std::cout << "ptr : " << ptr << std::endl;


}
