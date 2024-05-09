using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Runtime.InteropServices;
using System.Reflection;
using Debug=System.Diagnostics.Debug;
using System.Collections;



namespace CompressedFileManager
{
    using C_BOOL=int;


    internal class Test_DLL
    {
        const C_BOOL C_TRUE = 1;
        const C_BOOL C_FALSE = 0;

        #region WINAPI
        // LoadLibrary 함수
        [DllImport("kernel32.dll")]
        public static extern IntPtr LoadLibrary(string dllToLoad);

        // GetProcAddress 함수
        [DllImport("kernel32.dll")]
        public static extern IntPtr GetProcAddress(IntPtr hModule, string procedureName);

        // FreeLibrary 함수
        [DllImport("kernel32.dll")]
        public static extern bool FreeLibrary(IntPtr hModule);
        #endregion

        //---------------------------------------- Singleton ---------------------------------------- 
        #region Singleton
        static Test_DLL? _instance=null;
        public static Test_DLL Instance
        {
            get
            {
                if (_instance == null)
                    _instance = new Test_DLL();
                return _instance;
            }
        }
#pragma warning disable 8618
        private Test_DLL()
        {
        }
#pragma warning restore 8618
        #endregion

        private string targetDllPath = "ref/CFMCore.dll";

        ~Test_DLL()
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
        public DLL_Startup fn_Startup;
        public DLL_Cleanup fn_Cleanup;
        public DLL_Open fn_Open;
        public DLL_Close fn_Close;
        public DLL_GetFileCount fn_GetFileCount;
        public DLL_GetFile fn_GetFile;
        public DLL_DeleteFile fn_DeleteFile;
        public DLL_RevertDeleteFile fn_RevertDeleteFile;
        public DLL_IsChanged fn_IsChanged;
        public DLL_Recompress fn_Recompress;
        public DLL_PreviewFile fn_PreviewFile;
        public DLL_Preview_Release fn_Preview_Release;
        public DLL_Preview_GetType fn_Preview_GetType;
        public DLL_Preview_GetTmpPath fn_Preview_GetTmpPath;

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
                getFunc(out fn_DeleteFile              ,"DeleteFile");
                getFunc(out fn_RevertDeleteFile        ,"RevertDeleteFile");
                getFunc(out fn_IsChanged               ,"IsChanged");
                getFunc(out fn_Recompress              ,"Recompress");
                getFunc(out fn_PreviewFile             ,"PreviewFile");
                getFunc(out fn_Preview_Release         ,"Preview_Release");
                getFunc(out fn_Preview_GetType         ,"Preview_GetType");
                getFunc(out fn_Preview_GetTmpPath     , "Preview_GetTmpPath");
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

        private void getFunc<T>(out T result,string functionName) where T : Delegate
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
                result =Marshal.GetDelegateForFunctionPointer<T>(functionPointer);
            }
            catch(Exception ex)
            {
                Debug.WriteLine(ex.Message);
                throw new Exception(String.Format("Function {0} GetDelegateForFunctionPointer error", functionName));
            }
        }
        public bool Startup()
        {
            string targetDllPath = this.targetDllPath;
#if DEBUG
            DirectoryInfo currentDir = new DirectoryInfo(Environment.CurrentDirectory);
#pragma warning disable 8602
            Environment.CurrentDirectory = currentDir.Parent.Parent.Parent.FullName;
#pragma warning restore 8602
            targetDllPath = Path.Combine("../../../../", targetDllPath);
#endif
            // DLL을 로드
            hDll = LoadLibrary(targetDllPath);
            if (hDll == IntPtr.Zero)
            {
                Debug.WriteLine("Failed to load DLL");
                return false;
            }

            if (!loadFunction())
                return false;

            return true;
            // DLL 해제
        }
        public void Cleanup()
        {
            if (hDll != IntPtr.Zero)
                FreeLibrary(hDll);
            hDll = IntPtr.Zero;

        }
        public void Test()
        {
            fn_Startup();

            IntPtr handle= IntPtr.Zero;
            IntPtr ptr;
            unsafe{ ptr = (IntPtr) (&handle); }
            string targetPath = "../TestData/TestData.zip";
            string resultPath = "../TestData/TestData5.zip";
            string? previewPath = null;
            if (fn_Open(ptr, targetPath) ==C_TRUE)
            {
                var count = fn_GetFileCount(ptr);
                Debug.WriteLine(String.Format("count : {0}",count));
                List<string> fileList= new ();
                for (int i = 0; i < count; i++)
                {
                    StringBuilder stringBuilder = new StringBuilder(200);
                    
                    if (fn_GetFile(ptr, i, stringBuilder, 200)==C_TRUE)
                    {
                        var str = stringBuilder.ToString();
                        Debug.WriteLine(String.Format("GetFile[{0}] result : {1}", i,str));
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
                if(fn_PreviewFile(ptr,pPreview, fileList[0])==C_TRUE)
                {
                    var type = fn_Preview_GetType(pPreview);
                    Debug.WriteLine(String.Format("fn_Preview_GetTmpPath result = {0}", type));

                    StringBuilder tmpPath = new StringBuilder(200);

                    if(fn_Preview_GetTmpPath(pPreview, tmpPath,200)==C_TRUE)
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
                if(previewPath!=null)
                    Debug.WriteLine(String.Format("preview file exist = {0}", Path.Exists(previewPath)));
                Debug.WriteLine(String.Format("Release Preview file"));
                fn_Preview_Release(pPreview);
                if (previewPath!=null)
                    Debug.WriteLine(String.Format("preview file exist = {0}", Path.Exists(previewPath)));


                if (fn_DeleteFile(ptr, fileList[0])==C_FALSE)
                {
                    Debug.WriteLine(String.Format("fn_DeleteFile fail"));
                }
                if (fn_Recompress(ptr, resultPath)==C_FALSE)
                {
                    Debug.WriteLine(String.Format("fn_Recompress fail"));
                }
            }



            fn_Close(ptr);

            if (previewPath != null)
                Debug.WriteLine(String.Format("preview file exist = {0}", Path.Exists(previewPath)));

            fn_Cleanup();

        }

    }
}
