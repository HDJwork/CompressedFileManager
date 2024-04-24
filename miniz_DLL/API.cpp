#include<windows.h>
#include"API.h"
#include"miniz-3.0.2/miniz.h"

#include<string>
#include<vector>
#include<stack>
#include<filesystem>

static constexpr int SIZE_STR = 200;
static constexpr BOOL BOOL_FALSE = 0;
static constexpr BOOL BOOL_TRUE = 1;

enum eErrorCode
{
	ERROR_NONE = 0,
	ERROR_READER_INIT_FAIL,
	ERROR_READER_FILESTAT_INIT_FAIL,
	ERROR_READER_GET_FILENAME_FAIL,
	ERROR_READER_RESULT_GET_FILENAME_INDEX_OUT_OF_RANGE,
	ERROR_READER_RESULT_GET_FILENAME_BUFFER_IS_SMALL,
};

struct OutputData_Read {
	eErrorCode errorcode;
	int count;
	std::vector<std::string> fileList;
};

std::string multibyte_to_utf8(const std::string& str)
{
	int nLen = static_cast<int>(str.size());
	wchar_t warr[SIZE_STR];
	MultiByteToWideChar(CP_ACP, 0, (LPCSTR)str.c_str(), -1, warr, SIZE_STR);
	char carr[SIZE_STR];
	memset(carr, '\0', sizeof(carr));
	WideCharToMultiByte(CP_UTF8, 0, warr, -1, carr, SIZE_STR, NULL, NULL);
	return carr;
}
std::string utf8_to_multibyte(const std::string& str)
{
	wchar_t warr[SIZE_STR];
	int nLen = static_cast<int>(str.size());
	memset(warr, '\0', sizeof(warr));
	MultiByteToWideChar(CP_UTF8, 0, str.c_str(), -1, warr, SIZE_STR);
	char carr[SIZE_STR];    memset(carr, '\0', sizeof(carr));
	WideCharToMultiByte(CP_ACP, 0, warr, -1, carr, SIZE_STR, NULL, NULL);    return carr;
}


BOOL MINIZ_LIB_Read(PTR* _result, const char* buff)
{
	OutputData_Read& result = *new OutputData_Read();
	*_result = reinterpret_cast<PTR> (& result);
	mz_zip_archive archive;
	memset(&archive, 0, sizeof(mz_zip_archive));
	if (!mz_zip_reader_init_file(&archive, buff, 0))
	{
		result.errorcode = ERROR_READER_INIT_FAIL;
		return BOOL_FALSE;
	}
//	mz_zip_reader_init_file(&archive, buff, 0);

	result.count = mz_zip_reader_get_num_files(&archive);
	for (int i = 0; i < result.count; ++i)
	{
		mz_zip_archive_file_stat fileStat;
		if (!mz_zip_reader_file_stat(&archive, i, &fileStat))
		{
			result.errorcode = ERROR_READER_FILESTAT_INIT_FAIL;
			return BOOL_FALSE;
		}
		//if (!mz_zip_reader_is_file_encrypted(&archive, i))
		//	continue;
		char buff[SIZE_STR] = "";
		if (!mz_zip_reader_get_filename(&archive, i, buff, SIZE_STR))
		{
			result.errorcode = ERROR_READER_GET_FILENAME_FAIL;
			return BOOL_FALSE;
		}
		result.fileList.push_back(buff);
	}

	mz_zip_reader_end(&archive);
	return BOOL_TRUE;
}

int MINIZ_LIB_Read_Result_GetErrorCode(PTR* _result)
{
	OutputData_Read& result = *reinterpret_cast<OutputData_Read*>(*_result);
	return result.errorcode;
}

int MINIZ_LIB_Read_Result_GetCount(PTR* _result)
{
	OutputData_Read& result = *reinterpret_cast<OutputData_Read*>(*_result);
	return static_cast<int>(result.fileList.size());
}

BOOL MINIZ_LIB_Read_Result_GetFileName(PTR* _result, int index, char* buff, int buffCount)
{
	OutputData_Read& result = *reinterpret_cast<OutputData_Read*>(*_result);
	if (index >= result.fileList.size())
	{
		result.errorcode = ERROR_READER_RESULT_GET_FILENAME_INDEX_OUT_OF_RANGE;
		return BOOL_FALSE;
	}
	auto& target = result.fileList[index];
	if (target.size() >= buffCount)
	{
		result.errorcode = ERROR_READER_RESULT_GET_FILENAME_BUFFER_IS_SMALL;
		return BOOL_FALSE;
	}

	strcpy_s(buff, buffCount, target.c_str());
	return BOOL_TRUE;
}
BOOL MINIZ_LIB_Read_Result_GetFileName_UTF8(PTR* _result, int index, char* buff, int buffCount)
{
	OutputData_Read& result = *reinterpret_cast<OutputData_Read*>(*_result);
	if (index >= result.fileList.size())
	{
		result.errorcode = ERROR_READER_RESULT_GET_FILENAME_INDEX_OUT_OF_RANGE;
		return BOOL_FALSE;
	}
	auto& target = result.fileList[index];
	std::string str = target;
	str = multibyte_to_utf8(str);
	if (target.size() >= buffCount)
	{
		result.errorcode = ERROR_READER_RESULT_GET_FILENAME_BUFFER_IS_SMALL;
		return BOOL_FALSE;
	}

	strcpy_s(buff, buffCount, str.c_str());
	return BOOL_TRUE;

}

void MINIZ_LIB_Read_Result_Release(PTR* _result)
{
	if (*_result==0)
		return;
	OutputData_Read& result = *reinterpret_cast<OutputData_Read*>(*_result);
	try
	{

		delete& result;
	}
	catch (std::exception e)
	{
		//T.B.D
		*_result = 1;
	}
	*_result = 0;
}

BOOL MINIZ_LIB_InitDirectory(const char* path)
{
	std::filesystem::path targetPath = path;
	if (!targetPath.is_absolute())
		targetPath = std::filesystem::absolute(targetPath);
	auto bExist = std::filesystem::exists(targetPath);

	if (bExist && !std::filesystem::is_directory(targetPath))
		return BOOL_FALSE;

	//crate directory
	if (!bExist)
	{
		auto parent = targetPath;
		std::stack<std::filesystem::path> pathStack;
		while (!std::filesystem::exists(parent))
		{
			if (!parent.has_parent_path())
				return BOOL_FALSE;

			pathStack.push(parent);

			parent = parent.parent_path();
		}
		while (!pathStack.empty())
		{
			if(!std::filesystem::create_directory(pathStack.top()))
				return BOOL_FALSE;

			pathStack.pop();
		}
	}
	else
	{
		//check and remove children
		auto iter = std::filesystem::directory_iterator(targetPath);
		auto iter_end = std::filesystem::directory_iterator();

		std::vector<std::filesystem::path> childList;
		while (iter != iter_end)
		{
			childList.push_back(iter->path());
			++iter;
		}
		for (const auto& childPath : childList)
		{
			if (!std::filesystem::remove(childPath))
				return BOOL_FALSE;
		}
	}
	return BOOL_TRUE;
}
