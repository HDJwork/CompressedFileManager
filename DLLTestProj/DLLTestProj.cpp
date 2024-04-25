// DLLTestProj.cpp : 이 파일에는 'main' 함수가 포함됩니다. 거기서 프로그램 실행이 시작되고 종료됩니다.
//

#include <iostream>
#include <vector>
#include"../miniz_DLL/API.h"

#include<windows.h>

std::string multibyte_to_utf8(const std::string& str)
{
    int nLen = static_cast<int>(str.size());
    wchar_t warr[200];
    MultiByteToWideChar(CP_ACP, 0, (LPCSTR)str.c_str(), -1, warr, 200);
    char carr[200];
    memset(carr, '\0', sizeof(carr));
    WideCharToMultiByte(CP_UTF8, 0, warr, -1, carr, 200, NULL, NULL);
    return carr;
}

int main()
{
    std::cout << "Hello World!\n";

    std::vector<std::string> _v = {
        multibyte_to_utf8("1.txt")
        ,multibyte_to_utf8("B\\ㅂㅂㅂ.txt")
        ,multibyte_to_utf8("B\\1.txt") 
    };
    std::vector<const char*> v;
    char buff[200];

    PTR _ptr = 0;
    PTR *ptr = &_ptr;
    
    //auto filePath = "D:/Develop/CompressedFileManager/testproj/TestData/ㄱㄴ/TestData.zip";
    //if (MINIZ_LIB_Read(ptr, filePath))
    auto _filePath = multibyte_to_utf8("D:/Develop/CompressedFileManager/testproj/TestData/ㄱㄴ/TestData.zip");
    auto filePath = _filePath.c_str();
    if (MINIZ_LIB_Read_UTF8(ptr, filePath))
    {
        std::cout << "ptr : " << ptr << std::endl;
        int count = MINIZ_LIB_Read_Result_GetCount(ptr);
        std::cout << "count : " << count << std::endl;
        std::cout << "file list : " << std::endl;
        for (int i = 0; i < count; ++i)
        {
            if (!MINIZ_LIB_Read_Result_GetFileName(ptr, i, buff, 200))
            {
                std::cout << "MINIZ_LIB_Read_Result_GetFileName Fail!" << ptr << std::endl;
                break;
            }
            std::cout << buff << std::endl;
        }

        PTR _ptr2 = 0;
        PTR* ptr2 = &_ptr2;
        if (MINIZ_LIB_Preview_UTF8(ptr2, ptr, _v[1].c_str()))
        {
            if (!MINIZ_LIB_Preview_Result_GetTempFilePath(ptr2, buff, 200))
                std::cout << "MINIZ_LIB_Preview_Result_GetTempFilePath Fail!" << ptr << std::endl;
            else
                std::cout << buff << std::endl;
        }
        else
        {
            std::cout << "Preview fail!" << std::endl;
            std::cout << "error code : " << MINIZ_LIB_Preview_Result_GetErrorCode(ptr2) << std::endl;
        }
        MINIZ_LIB_Preview_Result_Release(ptr2);

    }
    else
    {
        std::cout << "read fail!" << std::endl;
        std::cout << "error code : "<<MINIZ_LIB_Read_Result_GetErrorCode(ptr) << std::endl;

    }
    MINIZ_LIB_Read_Result_Release(ptr);
    std::cout << "ptr : " << ptr << std::endl;

    auto _outputPath = multibyte_to_utf8("D:\\Develop\\CompOutput\\ㄱㄴ");
    auto outputPath = _outputPath.c_str();
    if (!MINIZ_LIB_InitDirectory_CleanUp_UTF8(outputPath))
        std::cout << "fail!" << std::endl;
    if (!MINIZ_LIB_Unzip_UTF8(filePath, outputPath))
        std::cout << "fail!" << std::endl;
    auto _zipPath = multibyte_to_utf8("D:/Develop/CompressedFileManager/testproj/TestData/ㄷㄹ/TestData2.zip");
    auto zipPath = _zipPath.c_str();
    if (!MINIZ_LIB_Zip_UTF8(outputPath, zipPath, nullptr, 0))
        std::cout << "fail!" << std::endl;

    auto _zipPath2 = multibyte_to_utf8("D:/Develop/CompressedFileManager/testproj/TestData/ㄷㄹ/TestData3.zip");
    auto zipPath2 = _zipPath2.c_str();
    v = { _v[0].c_str(),_v[1].c_str()};
    if (!MINIZ_LIB_Zip_UTF8(outputPath, zipPath2, v.data(), static_cast<int>(v.size())))
        std::cout << "fail!" << std::endl;
    auto _zipPath3 = multibyte_to_utf8("D:/Develop/CompressedFileManager/testproj/TestData/ㄷㄹ/TestData4.zip");
    auto zipPath3 = _zipPath3.c_str();
    /*std::vector<const char*>*/ v = { _v[0].c_str() };
    if (!MINIZ_LIB_Recompress_UTF8(filePath, zipPath3, v.data(), static_cast<int>(v.size())))
        std::cout << "fail!" << std::endl;


    //auto outputPath = "D:\\Develop\\CompOutput\\ㄱㄴ";
    //if (!MINIZ_LIB_InitDirectory_CleanUp(outputPath))
    //    std::cout << "fail!" << std::endl;
    //if (!MINIZ_LIB_Unzip_(filePath, outputPath))
    //    std::cout << "fail!" << std::endl;
    //auto zipPath = "D:/Develop/CompressedFileManager/testproj/TestData/ㄷㄹ/TestData2.zip";
    //if (!MINIZ_LIB_Zip(outputPath, zipPath, nullptr, 0))
    //    std::cout << "fail!" << std::endl;

    //auto zipPath2 = "D:/Develop/CompressedFileManager/testproj/TestData/ㄷㄹ/TestData3.zip";
    //std::vector<const char*> v = { "1.txt","B\\ㅂㅂㅂ.txt" };
    //if (!MINIZ_LIB_Zip(outputPath, zipPath2, v.data(), static_cast<int>(v.size())))
    //    std::cout << "fail!" << std::endl;
    //auto zipPath3 = "D:/Develop/CompressedFileManager/testproj/TestData/ㄷㄹ/TestData4.zip";
    ///*std::vector<const char*>*/ v = { "B\\1.txt" };
    //if (!MINIZ_LIB_Recompress(filePath, zipPath3, v.data(), static_cast<int>(v.size())))
    //    std::cout << "fail!" << std::endl;
}
