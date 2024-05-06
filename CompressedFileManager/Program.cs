namespace CompressedFileManager
{
    internal static class Program
    {
        /// <summary>
        ///  The main entry point for the application.
        /// </summary>
        [STAThread]
        static void Main()
        {
            var dll = Test_DLL.Instance;
            if(dll.Startup() == false)
            {
                dll.Cleanup();
                Application.Exit();
                return;
            }
            dll.Test();
            dll.Cleanup();


            // To customize application configuration such as set high DPI settings or default font,
            // see https://aka.ms/applicationconfiguration.
            ApplicationConfiguration.Initialize();
            Application.Run(new MainForm());
            //

        }
    }
}