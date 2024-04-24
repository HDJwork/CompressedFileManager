#include<windows.h>
#include"API.h"
#include"miniz-3.0.2/miniz.h"

#include<string>
#include<vector>
#include<stack>
#include<filesystem>
#include<fstream>
#include<algorithm>

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
	eErrorCode errorcode = ERROR_NONE;
	int count = 0;
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

BOOL checkAndCreateDirectoryImpl(const char* path, bool bCleanUp)
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
			if (!std::filesystem::create_directory(pathStack.top()))
				return BOOL_FALSE;

			pathStack.pop();
		}
	}
	else if (bCleanUp)
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
			if (!std::filesystem::remove_all(childPath))
				return BOOL_FALSE;
		}
	}
	return BOOL_TRUE;
}

std::vector<std::string> getSubFileList(std::filesystem::path targetDir)
{
	if (!targetDir.is_absolute())
		targetDir = std::filesystem::absolute(targetDir);
	std::vector<std::string> retval;

	auto iter = std::filesystem::directory_iterator(targetDir);
	auto iter_end = std::filesystem::directory_iterator();
	while (iter != iter_end)
	{
		auto subPath = iter->path();
		if (std::filesystem::is_directory(subPath))
		{
			auto result = getSubFileList(subPath);
			retval.insert(retval.end(), result.begin(), result.end());
		}
		else
			retval.push_back(subPath.string());
		++iter;
	}
	return retval;
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
	return checkAndCreateDirectoryImpl(path, false);
}

BOOL MINIZ_LIB_InitDirectory_CleanUp(const char* path)
{
	return checkAndCreateDirectoryImpl(path, true);
}

BOOL MINIZ_LIB_InitDirectory_UTF8(const char* _path)
{
	std::string path = utf8_to_multibyte(_path);
	return checkAndCreateDirectoryImpl(path.c_str(), false);
}

BOOL MINIZ_LIB_InitDirectory_CleanUp_UTF8(const char* _path)
{
	std::string path = utf8_to_multibyte(_path);
	return checkAndCreateDirectoryImpl(path.c_str(), true);
}

BOOL MINIZ_LIB_Unzip(const char* target, const char* resultPath)
{
	if (!std::filesystem::exists(target))
		return BOOL_FALSE;

	//read file to memory
	std::ifstream ifs(target,std::ios::binary);
	if (!ifs.is_open())
		return BOOL_FALSE;

	ifs.seekg(0, std::ios::end);
	size_t size = ifs.tellg();
	std::vector<char> buffer;
	buffer.resize(size);
	ifs.seekg(0);
	ifs.read(&buffer[0], size);

	ifs.close();

	//unzip process
	std::vector<char> fileBuffer;

	mz_zip_archive zip_archive;
	memset(&zip_archive, 0, sizeof(zip_archive));

	if (!mz_zip_reader_init_mem(&zip_archive, buffer.data(), size, 0)) {
		return BOOL_FALSE;
	}

	int num_files = mz_zip_reader_get_num_files(&zip_archive);
	for (int i = 0; i < num_files; ++i) {
		mz_zip_archive_file_stat file_stat;
		if (!mz_zip_reader_file_stat(&zip_archive, i, &file_stat)) {
			//printf("Error: Failed to read file info from zip.\n");
			mz_zip_reader_end(&zip_archive);
			return BOOL_FALSE;
		}

		auto fileSize = file_stat.m_uncomp_size;
		fileBuffer.resize(fileSize);
		
		if (!mz_zip_reader_extract_to_mem(&zip_archive, i, fileBuffer.data(), file_stat.m_uncomp_size, 0)) {
			//printf("Error: Failed to extract file from zip.\n");
			mz_zip_reader_end(&zip_archive);
			return BOOL_FALSE;
		}
		std::filesystem::path outputPath = resultPath;
		outputPath /= file_stat.m_filename;
		
		//create sub directory
		if (outputPath.has_parent_path())
			checkAndCreateDirectoryImpl(outputPath.parent_path().string().c_str(), false);
		
		//create file
		std::ofstream ofs(outputPath, std::ios::binary);
		if (!ofs.is_open())
		{
			mz_zip_reader_end(&zip_archive);
			return BOOL_FALSE;
		}
		if (!ofs.write(fileBuffer.data(), fileSize).good())
		{
			mz_zip_reader_end(&zip_archive);
			return BOOL_FALSE;
		}
		ofs.close();
	}

	mz_zip_reader_end(&zip_archive);

	return BOOL_TRUE;
}

BOOL MINIZ_LIB_Unzip_UTF8(const char* _target, const char* _resultPath)
{
	std::string target = utf8_to_multibyte(_target);
	std::string resultPath = utf8_to_multibyte(_resultPath);
	return MINIZ_LIB_Unzip(target.c_str(), resultPath.c_str());
}

BOOL MINIZ_LIB_Zip(const char* _targetDir, const char* _resultPath, const char** _passingList, int noOfPassingList)
{
	std::filesystem::path targetDir = _targetDir;

	//create target path parent directory
	std::filesystem::path resultPath = _resultPath;
	if (resultPath.has_parent_path())
	{
		auto parentPath = resultPath.parent_path();
		if(!std::filesystem::exists(resultPath.parent_path()))
			checkAndCreateDirectoryImpl(parentPath.string().c_str(), false);
	}

	//passingList sort 후 이진탐색으로 확인
	std::vector<std::string> passingList;
	passingList.reserve(noOfPassingList);
	for (int i = 0; i < noOfPassingList; ++i)
	{
		std::filesystem::path passingFile = *(_passingList + i);
		if (passingFile.is_absolute())
			passingFile = passingFile.lexically_relative(targetDir);
		passingList.push_back(passingFile.string());
	}
	std::sort(passingList.begin(), passingList.end());

	mz_zip_archive zip_archive;
	memset(&zip_archive, 0, sizeof(zip_archive));
	if (!mz_zip_writer_init_file(&zip_archive, _resultPath, 0)) {
		//printf("Error: Failed to initialize zip writer.\n");
		return BOOL_FALSE;
	}

	auto subFileList = getSubFileList(targetDir);
	for(const auto& subFile: subFileList)
	{
		std::filesystem::path subPath = subFile;
		auto relativePath = subPath.lexically_relative(targetDir).string();

		//passingList에 존재하면 패스
		if (std::binary_search(passingList.begin(), passingList.end(), relativePath))
			continue;

		if (!mz_zip_writer_add_file(
			&zip_archive, 
			multibyte_to_utf8(relativePath).c_str(),
			multibyte_to_utf8(subFile).c_str(),
			NULL, 0, MZ_BEST_COMPRESSION))
		{
			mz_zip_writer_end(&zip_archive);
			return BOOL_FALSE;
		}
	}

	if (!mz_zip_writer_finalize_archive(&zip_archive)) {
		//printf("Error: Failed to finalize zip archive.\n");
		mz_zip_writer_end(&zip_archive);
		return BOOL_FALSE;
	}

	mz_zip_writer_end(&zip_archive);
	return BOOL_TRUE;
}

BOOL MINIZ_LIB_Zip_UTF8(const char* _targetDir, const char* _resultPath, const char** _passingList, int noOfPassingList)
{
	std::string targetDir = utf8_to_multibyte(_targetDir);
	std::string resultPath = utf8_to_multibyte(_resultPath);
	std::vector<const char*> passingList;
	std::vector<std::string> passingList_Convert;
	passingList.reserve(noOfPassingList);
	passingList_Convert.reserve(noOfPassingList);
	for (int i = 0; i < noOfPassingList; ++i)
	{
		passingList_Convert.push_back(utf8_to_multibyte(*(_passingList + i)));
		passingList.push_back(passingList_Convert[i].c_str());
	}

	return MINIZ_LIB_Zip(targetDir.c_str(), resultPath.c_str(), passingList.data(), noOfPassingList);
}

BOOL MINIZ_LIB_Recompress_SetTmpFolder(const char* target, const char* resultPath, const char* tmpPath, const char** passingList, int noOfPassingList)
{
	if(!MINIZ_LIB_InitDirectory_CleanUp(tmpPath))
		return BOOL_FALSE;
	if (!MINIZ_LIB_Unzip(target, tmpPath))
		return BOOL_FALSE;
	if(!MINIZ_LIB_Zip(tmpPath, resultPath, passingList, noOfPassingList))
		return BOOL_FALSE;
	return BOOL_TRUE;

}

DLL_API BOOL MINIZ_LIB_Recompress_SetTmpFolder_UTF8(const char* target, const char* resultPath, const char* tmpPath, const char** passingList, int noOfPassingList)
{
	if (!MINIZ_LIB_InitDirectory_CleanUp_UTF8(tmpPath))
		return BOOL_FALSE;
	if (!MINIZ_LIB_Unzip_UTF8(target, tmpPath))
		return BOOL_FALSE;
	if (!MINIZ_LIB_Zip_UTF8(tmpPath, resultPath, passingList, noOfPassingList))
		return BOOL_FALSE;
	return BOOL_TRUE;
}

BOOL MINIZ_LIB_Recompress(const char* target, const char* resultPath, const char** passingList, int noOfPassingList)
{
	char buff[SIZE_STR];
	GetTempPathA(SIZE_STR, buff);
	std::filesystem::path tmpPath = buff;
	tmpPath /= "TEMP_MINIZ";
	BOOL retval = MINIZ_LIB_Recompress_SetTmpFolder(target, resultPath, tmpPath.string().c_str(), passingList, noOfPassingList);
	if (!std::filesystem::remove_all(tmpPath))
	{
		//Do nothing
	}
	return retval;
}

BOOL MINIZ_LIB_Recompress_UTF8(const char* target, const char* resultPath, const char** passingList, int noOfPassingList)
{
	char buff[SIZE_STR];
	GetTempPathA(SIZE_STR, buff);
	std::filesystem::path tmpPath = buff;
	tmpPath /= "TEMP_MINIZ";
	BOOL retval = MINIZ_LIB_Recompress_SetTmpFolder_UTF8(target, resultPath, tmpPath.string().c_str(), passingList, noOfPassingList);
	if (!std::filesystem::remove_all(tmpPath))
	{
		//Do nothing
	}
	return retval;
}
