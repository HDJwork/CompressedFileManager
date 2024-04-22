#include"API.h"
#include"miniz-3.0.2/miniz.h"

#include<string>
#include<vector>

enum eErrorCode
{
	ERROR_NONE = 0,
	ERROR_READER_INIT_FAIL,
	ERROR_READER_FILESTAT_INIT_FAIL,
	ERROR_READER_GET_FILENAME_FAIL,
};

struct OutputData_Read {
	eErrorCode errorcode;
	int count;
	std::vector<std::string> fileList;
};

BOOL MINIZ_LIB_Read(PTR* _result, const char* buff)
{
	OutputData_Read& result = *new OutputData_Read();
	*_result = reinterpret_cast<PTR> (& result);
	mz_zip_archive archive;
	memset(&archive, 0, sizeof(mz_zip_archive));
	if (!mz_zip_reader_init_file(&archive, buff, 0))
	{
		result.errorcode = ERROR_READER_INIT_FAIL;
		return false;
	}
//	mz_zip_reader_init_file(&archive, buff, 0);

	result.count = mz_zip_reader_get_num_files(&archive);
	for (int i = 0; i < result.count; ++i)
	{
		mz_zip_archive_file_stat fileStat;
		if (!mz_zip_reader_file_stat(&archive, i, &fileStat))
		{
			result.errorcode = ERROR_READER_FILESTAT_INIT_FAIL;
			return false;
		}
		if (!mz_zip_reader_is_file_encrypted(&archive, i))
			continue;
		static constexpr int SIZE_NAME = 200;
		char buff[SIZE_NAME] = "";
		if (!mz_zip_reader_get_filename(&archive, i, buff, SIZE_NAME))
		{
			result.errorcode = ERROR_READER_GET_FILENAME_FAIL;
			return false;
		}
		result.fileList.push_back(buff);
	}

	mz_zip_reader_end(&archive);
	return true;
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
	if (result.fileList.size() >= index)
		return false;
	auto& target = result.fileList[index];
	if (buffCount > target.size())
		return false;
	strcpy_s(buff, buffCount, target.c_str());
	return true;
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
