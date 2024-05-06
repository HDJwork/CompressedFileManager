using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Runtime.InteropServices;
using System.Reflection;
using Debug=System.Diagnostics.Debug;

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

        private IntPtr hDll = IntPtr.Zero;

        public delegate void DLL_Startup();
        public delegate void DLL_Cleanup();
        public delegate C_BOOL DLL_Open(IntPtr ptr, string path);
        public delegate void DLL_Close(IntPtr ptr);
        public delegate int DLL_GetFileCount(IntPtr ptr);
        public delegate C_BOOL DLL_GetFile(IntPtr ptr, int index, [MarshalAs(UnmanagedType.LPUTF8Str)] StringBuilder output, int bufferSize);

#pragma warning disable 0169
        public DLL_Startup fn_Startup;
        public DLL_Cleanup fn_Cleanup;
        public DLL_Open fn_Open;
        public DLL_Close fn_Close;
        public DLL_GetFileCount fn_GetFileCount;
        public DLL_GetFile fn_GetFile;
#pragma warning restore 0169


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

            try
            {
                getFunc(out fn_Startup, "Startup");
                getFunc(out fn_Cleanup, "Cleanup");
                getFunc(out fn_Open, "Open");
                getFunc(out fn_Close, "Close");
                getFunc(out fn_GetFileCount, "GetFileCount");
                getFunc(out fn_GetFile, "GetFile");
            }
            catch (Exception ex)
            {
                Debug.WriteLine(ex.Message);
                return false;
            }

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
            if(fn_Open(ptr, "../TestData/TestData.zip") ==C_TRUE)
            {
                var count = fn_GetFileCount(ptr);
                Debug.WriteLine(String.Format("count : {0}",count));
                for (int i = 0; i < count; i++)
                {
                    StringBuilder stringBuilder = new StringBuilder(200);
                    


                    if (fn_GetFile(ptr, i, stringBuilder, 200)==C_TRUE)
                    {
                        //StringBuilder tmp=new StringBuilder();
                        //for(int j=0;j<3;++j)
                        //{
                        //    for(int k=0;k<8;++k)
                        //    {
                        //        int index = j * 8 + k;
                        //        if (index >= stringBuilder.Length)
                        //            break;
                        //        tmp.Append(((byte)stringBuilder[index]).ToString() + " ");
                        //    }
                        //    tmp.AppendLine();
                        //}
                        //Debug.WriteLine(tmp);

                        var str = stringBuilder.ToString();
                        Debug.WriteLine(String.Format("GetFile[{0}] result : {1}", i,str));
                    }
                    else
                    {
                        Debug.WriteLine(String.Format("GetFile fail, index : {0}", i));
                        break;
                    }
                }
            }
            fn_Close(ptr);

            fn_Cleanup();

        }

    }
}
