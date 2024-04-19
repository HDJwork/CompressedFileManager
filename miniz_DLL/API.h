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

DLL_API BOOL MINIZ_LIB_Read(PTR* _result, const char* buff);
DLL_API int MINIZ_LIB_Read_Result_GetErrorCode(PTR* _result);
DLL_API int MINIZ_LIB_Read_Result_GetCount(PTR* _result);
DLL_API BOOL MINIZ_LIB_Read_Result_GetFileName(PTR* _result, int index, char* buff, int buffCount);
DLL_API void MINIZ_LIB_Read_Result_Release(PTR* _result);

}