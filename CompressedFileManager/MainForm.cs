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
            this.AllowDrop = true; // ���� �巡�� �� ����� ���� �� �ֵ��� ����
            this.DragEnter += MainForm_DragEnter; // �巡�� ���� �̺�Ʈ �ڵ鷯 ���
            this.DragDrop += MainForm_DragDrop; // ��� �̺�Ʈ �ڵ鷯 ���
            updateUI();
        }

        private void MainForm_DragEnter(object? sender, DragEventArgs e)
        {
            // �巡�׵� �����Ͱ� �������� Ȯ��
            if (e.Data == null)
                return;
            if (e.Data.GetDataPresent(DataFormats.FileDrop))
            {
                e.Effect = DragDropEffects.Copy; // ����� ȿ�� ����
            }
            else
            {
                e.Effect = DragDropEffects.None; // �ٸ� ��쿡�� ȿ�� �������� ����
            }
        }

        private void MainForm_DragDrop(object? sender, DragEventArgs e)
        {
            if (e.Data == null)
                return;
            // �巡�׵� �����Ͱ� �������� Ȯ��
            if (e.Data.GetDataPresent(DataFormats.FileDrop))
            {
                // ���� ��� �迭 ��������
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

            // FileOpenDialog �ν��Ͻ� ����
            OpenFileDialog openFileDialog = new OpenFileDialog();

            // ��ȭ ���� ����
            openFileDialog.Title = "���� ����"; // ��ȭ ���� ����
            openFileDialog.Filter = "���� ���� (*.zip)|*.zip|��� ���� (*.*)|*.*"; // ���� ����

            // ��ȭ ���ڸ� ǥ���ϰ� ����ڰ� OK�� Ŭ���ߴ��� Ȯ��
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
