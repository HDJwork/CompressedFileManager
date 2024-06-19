using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;
using System.Reflection.Metadata;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;
using System.Runtime.ExceptionServices;
using System.Security;

namespace CompressedFileManager
{
    using static CompressedFileManager.CFMDllWrapper;
    using C_BOOL = int;
    internal class CFMDllWrapper
    {
        internal const C_BOOL C_TRUE = 1;
        internal const C_BOOL C_FALSE = 0;
        internal const int DefaultBufferCount = 200;

        #region WINAPI
        // LoadLibrary 함수
        [DllImport("kernel32.dll")]
        public static extern IntPtr LoadLibrary(string dllToLoad);

        // GetProcAddress 함수
        [DllImport("kernel32.dll")]
        public static extern IntPtr GetProcAddress(IntPtr hModule, string procedureName);

        // FreeLibrary 함수
        [DllImport("kernel32.dll")]
        [return: MarshalAs(UnmanagedType.Bool)]
        public static extern bool FreeLibrary(IntPtr hModule);


        #endregion

        //---------------------------------------- Singleton ---------------------------------------- 
        #region Singleton
        static CFMDllWrapper? _instance = null;
        public static CFMDllWrapper Instance
        {
            get
            {
                if (_instance == null)
                    _instance = new CFMDllWrapper();
                return _instance;
            }
        }
#pragma warning disable 8618
        private CFMDllWrapper()
        {
        }
#pragma warning restore 8618
        #endregion

        private string targetDllFileName = "CFMCore.dll";

        //private static readonly Destructor Finalise = new Destructor();
        //private sealed class Destructor
        //{
        //    ~Destructor()
        //    {
        //        CFMDllWrapper.Instance.Cleanup();
        //        // One time only destructor.
        //    }
        //}
        ~CFMDllWrapper()
        {
            Cleanup();
        }


        //---------------------------------------- DLL definition ---------------------------------------- 
        #region DLL Definition

        public delegate void DLL_Startup();
        public delegate void DLL_Cleanup();
        public delegate C_BOOL DLL_Open(IntPtr out_pCompressedFile, [MarshalAs(UnmanagedType.LPUTF8Str)] string path);
        public delegate void DLL_Close(IntPtr pCompressedFile);
        public delegate int DLL_GetFileCount(IntPtr pCompressedFile);
        public delegate C_BOOL DLL_GetFile(IntPtr pCompressedFile, int index, [MarshalAs(UnmanagedType.LPUTF8Str)] StringBuilder output, int bufferSize);
        public delegate C_BOOL DLL_DeleteFile(IntPtr pCompressedFile, [MarshalAs(UnmanagedType.LPUTF8Str)] string file);
        public delegate C_BOOL DLL_RevertDeleteFile(IntPtr pCompressedFile, [MarshalAs(UnmanagedType.LPUTF8Str)] string file);
        public delegate C_BOOL DLL_IsChanged(IntPtr pCompressedFile);
        public delegate C_BOOL DLL_Recompress(IntPtr pCompressedFile, [MarshalAs(UnmanagedType.LPUTF8Str)] string targetPath);
        public delegate C_BOOL DLL_PreviewFile(IntPtr pCompressedFile, IntPtr out_pPreview, [MarshalAs(UnmanagedType.LPUTF8Str)] string targetPath);
        public delegate void DLL_Preview_Release(IntPtr pPreview);
        public delegate int DLL_Preview_GetType(IntPtr pPreview);
        public delegate int DLL_Preview_GetTmpPath(IntPtr pPreview, [MarshalAs(UnmanagedType.LPUTF8Str)] StringBuilder output, int bufferSize);

#pragma warning disable 0169
        internal DLL_Startup fn_Startup;
        internal DLL_Cleanup fn_Cleanup;
        internal DLL_Open fn_Open;
        internal DLL_Close fn_Close;
        internal DLL_GetFileCount fn_GetFileCount;
        internal DLL_GetFile fn_GetFile;
        internal DLL_DeleteFile fn_DeleteFile;
        internal DLL_RevertDeleteFile fn_RevertDeleteFile;
        internal DLL_IsChanged fn_IsChanged;
        internal DLL_Recompress fn_Recompress;
        internal DLL_PreviewFile fn_PreviewFile;
        internal DLL_Preview_Release fn_Preview_Release;
        internal DLL_Preview_GetType fn_Preview_GetType;
        internal DLL_Preview_GetTmpPath fn_Preview_GetTmpPath;

#pragma warning restore 0169
        private bool loadFunction()
        {
            try
            {
                getFunc(out fn_Startup, "Startup");
                getFunc(out fn_Cleanup, "Cleanup");
                getFunc(out fn_Open, "Open");
                getFunc(out fn_Close, "Close");
                getFunc(out fn_GetFileCount, "GetFileCount");
                getFunc(out fn_GetFile, "GetFile");
                getFunc(out fn_DeleteFile, "DeleteFile");
                getFunc(out fn_RevertDeleteFile, "RevertDeleteFile");
                getFunc(out fn_IsChanged, "IsChanged");
                getFunc(out fn_Recompress, "Recompress");
                getFunc(out fn_PreviewFile, "PreviewFile");
                getFunc(out fn_Preview_Release, "Preview_Release");
                getFunc(out fn_Preview_GetType, "Preview_GetType");
                getFunc(out fn_Preview_GetTmpPath, "Preview_GetTmpPath");
            }
            catch (Exception ex)
            {
                Debug.WriteLine(ex.Message);
                return false;
            }
            return true;
        }
        #endregion


        private IntPtr hDll = IntPtr.Zero;
        private bool bStartup = false;
        private object lockObj = new object();

        private void getFunc<T>(out T result, string functionName) where T : Delegate
        {
            // DLL에서 사용할 함수의 포인터를 가져옴
            IntPtr functionPointer = GetProcAddress(hDll, functionName);
            if (functionPointer == IntPtr.Zero)
            {
                Debug.WriteLine("Failed to get function pointer");

                throw new Exception(String.Format("Function {0} not Found", functionName));
            }

            // 함수 포인터를 대리자로 변환
            try
            {
                result = Marshal.GetDelegateForFunctionPointer<T>(functionPointer);
            }
            catch (Exception ex)
            {
                Debug.WriteLine(ex.Message);
                throw new Exception(String.Format("Function {0} GetDelegateForFunctionPointer error", functionName));
            }
        }
        public bool IsStartup() {  return bStartup; }
        internal bool Startup()
        {
            if (bStartup)
                return true;
            if (hDll != IntPtr.Zero)
            {
                return true;
                //Cleanup();
            }

            // DLL을 로드
            List<string> dllPathList = new List<string>();

            DirectoryInfo currentDir = new DirectoryInfo(Environment.CurrentDirectory);
            //project dir(CompressFileManager/CompressFileManager)
            string targetDir = currentDir.Parent?.Parent?.Parent?.FullName ?? "";

            //for vs debugger run
            if(targetDir!="")
            {
                string dir = Path.GetFileName(targetDir) ?? "";
                if (dir == "CompressedFileManager")
                    Environment.CurrentDirectory = targetDir;
            }
#if DEBUG
            dllPathList.Add("../ref/debug/" + this.targetDllFileName);
            dllPathList.Add("ref/" + this.targetDllFileName);
#else

            dllPathList.Add("ref/" + this.targetDllFileName);
            dllPathList.Add("../ref/release" + this.targetDllFileName);

#endif
            bool bScuccess = false;
            foreach(string dllPath in dllPathList)
            {

                hDll = LoadLibrary(dllPath);
                if (hDll != IntPtr.Zero)
                {
                    bScuccess = true;
                    break;
                }
            }
            if (!bScuccess)
            {
                Debug.WriteLine("Failed to load DLL");
                return false;
            }

            if (!loadFunction())
                return false;
            bStartup = true;
            return true;
            // DLL 해제
        }
        // just call once
        // call cleanup and call startup case error detected
        internal void Cleanup()
        {
            if (hDll != IntPtr.Zero)
            {
                if (FreeLibrary(hDll))
                {
                    hDll = IntPtr.Zero;
                    bStartup = false;
                    Thread.Sleep(2000);
                }
            }

        }
        internal void Lock() { Monitor.Enter(lockObj); }
        internal void Unlock() { Monitor.Exit(lockObj); }
#if DEBUG
        public static void Test()
        {
            var instance = CFMDllWrapper.Instance;
            instance.Startup();
            instance.fn_Startup();

            IntPtr handle = IntPtr.Zero;
            IntPtr ptr;
            unsafe { ptr = (IntPtr)(&handle); }
            string targetPath = "../TestData/TestData.zip";
            string resultPath = "../TestData/TestData5.zip";
            string? previewPath = null;
            if (instance.fn_Open(ptr, targetPath) == C_TRUE)
            {
                var count = instance.fn_GetFileCount(ptr);
                Debug.WriteLine(String.Format("count : {0}", count));
                List<string> fileList = new();
                for (int i = 0; i < count; i++)
                {
                    StringBuilder stringBuilder = new StringBuilder(200);

                    if (instance.fn_GetFile(ptr, i, stringBuilder, 200) == C_TRUE)
                    {
                        var str = stringBuilder.ToString();
                        Debug.WriteLine(String.Format("GetFile[{0}] result : {1}", i, str));
                        fileList.Add(str);
                    }
                    else
                    {
                        Debug.WriteLine(String.Format("GetFile fail, index : {0}", i));
                        break;
                    }
                }
                IntPtr handle_Preview = IntPtr.Zero;
                IntPtr pPreview;
                unsafe { pPreview = (IntPtr)(&handle_Preview); }
                if (instance.fn_PreviewFile(ptr, pPreview, fileList[0]) == C_TRUE)
                {
                    var type = instance.fn_Preview_GetType(pPreview);
                    Debug.WriteLine(String.Format("fn_Preview_GetTmpPath result = {0}", type));

                    StringBuilder tmpPath = new StringBuilder(200);

                    if (instance.fn_Preview_GetTmpPath(pPreview, tmpPath, 200) == C_TRUE)
                    {
                        previewPath = tmpPath.ToString();
                        Debug.WriteLine(String.Format("fn_Preview_GetTmpPath result = {0}", previewPath));
                    }
                    else
                    {
                        Debug.WriteLine(String.Format("fn_Preview_GetTmpPath fail"));
                    }
                }
                else
                {
                    Debug.WriteLine(String.Format("fn_PreviewFile fail"));
                }
                if (previewPath != null)
                    Debug.WriteLine(String.Format("preview file exist = {0}", Path.Exists(previewPath)));
                Debug.WriteLine(String.Format("Release Preview file"));
                instance.fn_Preview_Release(pPreview);
                if (previewPath != null)
                    Debug.WriteLine(String.Format("preview file exist = {0}", Path.Exists(previewPath)));


                if (instance.fn_DeleteFile(ptr, fileList[0]) == C_FALSE)
                {
                    Debug.WriteLine(String.Format("fn_DeleteFile fail"));
                }
                if (instance.fn_Recompress(ptr, resultPath) == C_FALSE)
                {
                    Debug.WriteLine(String.Format("fn_Recompress fail"));
                }
            }



            instance.fn_Close(ptr);

            if (previewPath != null)
                Debug.WriteLine(String.Format("preview file exist = {0}", Path.Exists(previewPath)));

            instance.fn_Cleanup();

            //instance.Cleanup();

        }
#endif
    }

    public class CFM_CompressedFile
    {
        private IntPtr handle = IntPtr.Zero;
        private Dictionary<int, CFM_PreviewFile> previewFileList= new Dictionary<int, CFM_PreviewFile>();
        public List<string> FileList { get; }
        public string FilePath { get; }
        private CFM_CompressedFile(IntPtr handle,string filePath, List<string> fileList)
        {
            this.handle = handle;
            FileList = fileList;
            FilePath= filePath;
        }
        ~CFM_CompressedFile()
        {
            previewFileList.Clear();
            GC.Collect();

            if (this.handle != IntPtr.Zero)
            {
                var dll = CFMDllWrapper.Instance;
                dll.Lock();
                for(int i=0;i<100;++i)
                try
                {
                    Debug.WriteLine($"CompressedFile Close : {handle:x}");
                        unsafe
                        {
                            fixed (IntPtr* _ptr = &this.handle)
                            {

                                dll.fn_Close((IntPtr)_ptr);
                            }
                        }
                        break;
                }
                catch (Exception ex){
                    Debug.WriteLine(ex.Message);
                }

                dll.Unlock();
                this.handle = IntPtr.Zero;
            }
        }
        public static void Startup() { CFMDllWrapper.Instance.Startup(); }
        public static void Cleanup() { CFMDllWrapper.Instance.Cleanup(); }

        public static CFM_CompressedFile? Open(string path)
        {
            var dll = CFMDllWrapper.Instance;
            IntPtr handle = IntPtr.Zero;
            IntPtr ptr;
            unsafe { ptr = (IntPtr)(&handle); }
            dll.Lock();
            var result = dll.fn_Open(ptr, path);
            Debug.WriteLine($"CompressedFile Open : {handle:x}"); 
            dll.Unlock();

            if (result == CFMDllWrapper.C_FALSE)
            {
                dll.Lock();
                dll.fn_Close(ptr);
                dll.Unlock();
                return null;
            }
            dll.Lock();
            int fileCount=dll.fn_GetFileCount(ptr);
            dll.Unlock();
            List<string> fileList = new List<string>();
            for(int i=0; i<fileCount;++i)
            {
                StringBuilder sb = new StringBuilder(CFMDllWrapper.DefaultBufferCount);
                if(dll.fn_GetFile(ptr,i,sb, CFMDllWrapper.DefaultBufferCount) == CFMDllWrapper.C_FALSE)
                {
                    dll.fn_Close(ptr);
                    return null;
                }
                fileList.Add(sb.ToString());
            }
            return new CFM_CompressedFile(handle, path, fileList);

        }

        public CFM_PreviewFile? Preview(int index)
        {
            if(index>= FileList.Count||index<0) 
                return null;

            string targetFile = FileList[index];

            CFM_PreviewFile? retval = null;
            if (previewFileList.TryGetValue(index,out retval))
            {
                return retval;
            }

            var dll = CFMDllWrapper.Instance;
            IntPtr handle = IntPtr.Zero;
            IntPtr ptr;
            unsafe { ptr = (IntPtr)(&handle); }
            IntPtr ptr_CompressHandle = IntPtr.Zero;
            unsafe
            {
                fixed (IntPtr* _ptr_CompressHandle = &this.handle)
                {
                    ptr_CompressHandle=(IntPtr)(_ptr_CompressHandle);
                }
            }
            dll.Lock();
            var result_Preview = dll.fn_PreviewFile(ptr_CompressHandle, ptr, targetFile);
            Debug.WriteLine($"Preview Open : {handle:x}"); 
            dll.Unlock();
            if (result_Preview == CFMDllWrapper.C_FALSE)
            {
                dll.Lock();
                dll.fn_Preview_Release(ptr);
                dll.Unlock();
                return null;
            }
            dll.Lock();
            int _type = dll.fn_Preview_GetType(ptr);
            dll.Unlock();
            CFM_PreviewFile.EType type;
            switch(_type)
            {
                case 0:type = CFM_PreviewFile.EType.Error; break; 
                case 1:type = CFM_PreviewFile.EType.Image; break; 
                case 2:type = CFM_PreviewFile.EType.Unknown; break;
                default: type = CFM_PreviewFile.EType.Error; break;
            }
            StringBuilder sb = new StringBuilder(CFMDllWrapper.DefaultBufferCount);
            dll.Lock();
            var result_GetTempPath = dll.fn_Preview_GetTmpPath(ptr, sb, CFMDllWrapper.DefaultBufferCount);
            dll.Unlock();
            if (result_GetTempPath == CFMDllWrapper.C_FALSE)
            {
                dll.Lock();
                dll.fn_Preview_Release(ptr);
                dll.Unlock();
                return null;
            }
            retval = new CFM_PreviewFile(this.handle, handle, targetFile, sb.ToString(), type);
            if(retval != null)
            {
                previewFileList.Add(index, retval);
            }

            return retval;
        }
        public bool DeleteFile(int index)
        {
            IntPtr ptr = IntPtr.Zero;
            unsafe
            {
                fixed (IntPtr* _ptr = &this.handle)
                {
                    ptr = (IntPtr)(_ptr);
                }
            }
            if (index >= FileList.Count || index < 0)
                return false;
            string targetFile = FileList[index];

            var dll = CFMDllWrapper.Instance;
            dll.Lock();
            var result = dll.fn_DeleteFile(ptr, targetFile);
            dll.Unlock();

            return result != CFMDllWrapper.C_FALSE;
        }
        public bool RevertDeleteFile(int index)
        {
            IntPtr ptr = IntPtr.Zero;
            unsafe
            {
                fixed (IntPtr* _ptr = &this.handle)
                {
                    ptr = (IntPtr)(_ptr);
                }
            }
            if (index >= FileList.Count || index < 0)
                return false;
            string targetFile = FileList[index];

            var dll = CFMDllWrapper.Instance;
            dll.Lock();
            var result = dll.fn_RevertDeleteFile(ptr, targetFile);
            dll.Unlock();
            return result != CFMDllWrapper.C_FALSE;
        }
        public bool Recompress()
        {
            return Recompress(FilePath);
        }
        public bool Recompress(string targetPath)
        {
            IntPtr ptr = IntPtr.Zero;
            unsafe
            {
                fixed (IntPtr* _ptr = &this.handle)
                {
                    ptr = (IntPtr)(_ptr);
                }
            }
            var dll = CFMDllWrapper.Instance;
            dll.Lock();
            var result = dll.fn_Recompress(ptr, targetPath);
            dll.Unlock();
            return result != CFMDllWrapper.C_FALSE; ;
        }
    }
    public class CFM_PreviewFile
    {
        public enum EType { Error,Image,Unknown,}
        private IntPtr handle = IntPtr.Zero;
        private IntPtr compressedFileHandle;
        public string FileName { get; }
        public string TmpPath { get; }
        public EType Type { get; }

        internal CFM_PreviewFile(IntPtr compressedFileHandle, IntPtr handle, string fileName, string tmpPath, EType type)
        {
            this.compressedFileHandle = compressedFileHandle;
            this.handle = handle;
            FileName = fileName;
            TmpPath = tmpPath;
            Type = type;
        }
        ~CFM_PreviewFile()
        {
            IntPtr ptr = IntPtr.Zero;
            unsafe
            {
                fixed (IntPtr* _ptr = &this.handle)
                {
                    ptr = (IntPtr)(_ptr);
                }
            }
            if (ptr != IntPtr.Zero)
            {
                var dll = CFMDllWrapper.Instance;

                dll.Lock();
                Debug.WriteLine($"Preview Close : {handle:x}");

                unsafe
                {
                    fixed (IntPtr* _ptr = &this.handle)
                    {
                        dll.fn_Preview_Release((IntPtr)_ptr);
                    }
                }
                dll.Unlock();
                ptr = IntPtr.Zero;
            }
        }

    }
}
