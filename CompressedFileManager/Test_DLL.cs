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


    internal class Test_DLL
    {
        public static void Test()
        {

            CFM_CompressedFile.Startup();

            string targetPath = "../TestData/TestData.zip";
            string resultPath = "../TestData/TestData5.zip";
            var compressedFile = CFM_CompressedFile.Open(targetPath);
            if(compressedFile!=null)
            {
                Debug.WriteLine("Open Success!");
                Debug.WriteLine("File List =>");
                foreach (var file in compressedFile.FileList)
                {
                    Debug.WriteLine(file);
                }
                var preview = compressedFile.Preview(0);
                if(preview!=null)
                {
                    Debug.WriteLine("Preview Success!");
                    Debug.WriteLine(String.Format("type : {0}, temp path : {1}", preview.Type, preview.TmpPath));
                }

                if(!compressedFile.DeleteFile(0))
                {
                    Debug.WriteLine("delete file fail!");

                }
                if (!compressedFile.Recompress(resultPath))
                {
                    Debug.WriteLine("Recompress fail!");

                }


                Debug.WriteLine("Test Success!");
            }

            CFM_CompressedFile.Cleanup();
        }

    }
}
