// DLLTestProj.cpp : 이 파일에는 'main' 함수가 포함됩니다. 거기서 프로그램 실행이 시작되고 종료됩니다.
//

#include <iostream>
#include <vector>
#include"../miniz_DLL/API.h"

int main()
{
    std::cout << "Hello World!\n";
    PTR _ptr = 0;
    PTR *ptr = &_ptr;
    
    auto filePath = "D:/Develop/CompressedFileManager/testproj/TestData/TestData.zip";
    if (MINIZ_LIB_Read(ptr, filePath))
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

    auto outputPath = "D:\\Develop\\CompOutput\\";
    MINIZ_LIB_InitDirectory_CleanUp(outputPath);
    MINIZ_LIB_Unzip(filePath, outputPath);
    auto zipPath = "D:/Develop/CompressedFileManager/testproj/TestData/TestData2.zip";
    MINIZ_LIB_Zip(outputPath, zipPath, nullptr, 0);
    auto zipPath2 = "D:/Develop/CompressedFileManager/testproj/TestData/TestData3.zip";
    std::vector<const char*> v = {"1.txt","B\\ㅂㅂㅂ.txt"};
    MINIZ_LIB_Zip(outputPath, zipPath2, v.data(), static_cast<int>(v.size()));

}
