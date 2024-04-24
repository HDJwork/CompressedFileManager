#pragma once
#define DLL_EXPORT
#ifdef DLL_EXPORT
#  define DLL_API __declspec(dllexport)
#else
#  define DLL_API __declspec(dllimport)
#endif

typedef int BOOL;
typedef unsigned long long PTR;

extern "C"
{

DLL_API BOOL MINIZ_LIB_Read(PTR* _result, const char* path);
DLL_API int MINIZ_LIB_Read_Result_GetErrorCode(PTR* _result);
DLL_API int MINIZ_LIB_Read_Result_GetCount(PTR* _result);
DLL_API BOOL MINIZ_LIB_Read_Result_GetFileName(PTR* _result, int index, char* buff, int buffCount);
DLL_API BOOL MINIZ_LIB_Read_Result_GetFileName_UTF8(PTR* _result, int index, char* buff, int buffCount);
DLL_API void MINIZ_LIB_Read_Result_Release(PTR* _result);

DLL_API BOOL MINIZ_LIB_InitDirectory(const char* path);
DLL_API BOOL MINIZ_LIB_InitDirectory_CleanUp(const char* path);
DLL_API BOOL MINIZ_LIB_InitDirectory_UTF8(const char* path);
DLL_API BOOL MINIZ_LIB_InitDirectory_CleanUp_UTF8(const char* path);
DLL_API BOOL MINIZ_LIB_Unzip(const char* target, const char* resultPath);
DLL_API BOOL MINIZ_LIB_Unzip_UTF8(const char* target, const char* resultPath);
DLL_API BOOL MINIZ_LIB_Zip(const char* targetDir, const char* resultPath, const char** passingList, int noOfPassingList);
DLL_API BOOL MINIZ_LIB_Zip_UTF8(const char* targetDir, const char* resultPath, const char** passingList, int noOfPassingList);
DLL_API BOOL MINIZ_LIB_Recompress_SetTmpFolder(const char* target, const char* resultPath, const char* tmpPath, const char** passingList, int noOfPassingList);
DLL_API BOOL MINIZ_LIB_Recompress_SetTmpFolder_UTF8(const char* target, const char* resultPath, const char* tmpPath, const char** passingList, int noOfPassingList);
DLL_API BOOL MINIZ_LIB_Recompress(const char* target, const char* resultPath, const char** passingList, int noOfPassingList);
DLL_API BOOL MINIZ_LIB_Recompress_UTF8(const char* target, const char* resultPath, const char** passingList, int noOfPassingList);


}