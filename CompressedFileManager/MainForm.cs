namespace CompressedFileManager
{
    public partial class MainForm : Form
    {
        private CFM_CompressedFile? compressedFile = null;

        private bool Open(string targetPath)
        {
            compressedFile = CFM_CompressedFile.Open(targetPath);
            if (compressedFile == null)
                return false;
            treeView.Nodes.Clear();
            GC.Collect();
            TreeNode root = new TreeNode(Path.GetFileName(targetPath));
            foreach (var file in compressedFile.FileList)
            {
                root.Nodes.Add(file);
            }
            treeView.Nodes.Add(root);

            updateUI();
            return true;
        }

        private bool isOpen { get { return compressedFile != null; } }

        private void updateUI()
        {
            bool bOpened = isOpen;
            if (!bOpened)
            {
                treeView.Nodes.Clear();
                pictureBox.Image = null;
            }
            button_Close.Enabled = bOpened;
            button_Recompress.Enabled = bOpened;
            button_RecompressAs.Enabled = bOpened;
            treeView.Enabled = bOpened;
            pictureBox.Enabled = bOpened;
        }
        private void selectItem(int index)
        {
            if (compressedFile == null)
                return;
            var preview = compressedFile.Preview(index);
            if (preview == null)
            {
                MessageBox.Show("Preview Fail!");

                return;
            }
            if (preview.Type == CFM_PreviewFile.EType.Image)
            {
                setImage(preview.TmpPath);
            }
        }
        private void showMessageBox_FileOpenFail()
        {
            MessageBox.Show("File Open Fail!");
        }
        private bool setImage(string path)
        {
            this.Invoke((string path) =>
            {
                try
                {
                    pictureBox.Image = Image.FromFile(path);
                }
                catch
                {
                    MessageBox.Show("Image Load Fail!");
                }

            }, path
            );

            return true;
        }

        public MainForm()
        {
            InitializeComponent();
            this.AllowDrop = true; // 폼이 드래그 앤 드롭을 받을 수 있도록 설정
            this.DragEnter += MainForm_DragEnter; // 드래그 진입 이벤트 핸들러 등록
            this.DragDrop += MainForm_DragDrop; // 드롭 이벤트 핸들러 등록
            updateUI();
        }

        private void MainForm_DragEnter(object? sender, DragEventArgs e)
        {
            // 드래그된 데이터가 파일인지 확인
            if (e.Data == null)
                return;
            if (e.Data.GetDataPresent(DataFormats.FileDrop))
            {
                e.Effect = DragDropEffects.Copy; // 복사로 효과 설정
            }
            else
            {
                e.Effect = DragDropEffects.None; // 다른 경우에는 효과 없음으로 설정
            }
        }

        private void MainForm_DragDrop(object? sender, DragEventArgs e)
        {
            if (e.Data == null)
                return;
            // 드래그된 데이터가 파일인지 확인
            if (e.Data.GetDataPresent(DataFormats.FileDrop))
            {
                // 파일 경로 배열 가져오기
                var data = e.Data.GetData(DataFormats.FileDrop);
                if (data == null)
                    return;

                var files = data as string[];
                if (files == null)
                    return;
                if (!Open(files[0]))
                {
                    showMessageBox_FileOpenFail();
                }
            }
        }

        private void button_Open_Click(object sender, EventArgs e)
        {

            // FileOpenDialog 인스턴스 생성
            OpenFileDialog openFileDialog = new OpenFileDialog();

            // 대화 상자 설정
            openFileDialog.Title = "파일 열기"; // 대화 상자 제목
            openFileDialog.Filter = "압축 파일 (*.zip)|*.zip|모든 파일 (*.*)|*.*"; // 필터 설정

            // 대화 상자를 표시하고 사용자가 OK를 클릭했는지 확인
            if (openFileDialog.ShowDialog() == DialogResult.OK)
            {
                if (!Open(openFileDialog.FileName))
                {
                    showMessageBox_FileOpenFail();
                }
            }
        }

        private void treeView_AfterSelect(object sender, TreeViewEventArgs e)
        {
            if (treeView.SelectedNode == null)
                return;
            if (e.Node == null)
                return;
            if (e.Node == treeView.Nodes[0])
                return;
            selectItem(e.Node.Index);
        }

        private void button_Exit_Click(object sender, EventArgs e)
        {
            Application.Exit();
        }

        private void button_Close_Click(object sender, EventArgs e)
        {
            compressedFile = null;
            GC.Collect();
            updateUI();

        }

        private void button_Recompress_Click(object sender, EventArgs e)
        {

        }

        private void button_RecompressAs_Click(object sender, EventArgs e)
        {

        }
    }
}
