using SixLabors.ImageSharp.Formats;
using System.Drawing.Imaging;
using System.Windows.Forms;

using SixLabors.ImageSharp;
//using SixLabors.ImageSharp.Formats;
using SixLabors.ImageSharp.Processing;
using SixLabors.ImageSharp.Formats.Webp;
using SixLabors.ImageSharp.Formats.Jpeg;
using SixLabors.ImageSharp.Formats.Png;
using System.Xml.Linq;
using static System.Windows.Forms.VisualStyles.VisualStyleElement;

namespace CompressedFileManager
{
    public partial class MainForm : Form
    {
        private CFM_CompressedFile? compressedFile = null;
        private string lastFile = "";
        private string currentImage = "";
        private int lastSelectIndex = -1;
        struct SizeContainer
        {
            public int Main_Top;
            public int Main_BottomOffset;
            public int Bottom_TopOffset;
            public int Main_RightOffset;
        }
        private SizeContainer sizeContainer = new SizeContainer();

        public MainForm()
        {
            InitializeComponent();
            this.AllowDrop = true; // ���� �巡�� �� ����� ���� �� �ֵ��� ����
            this.DragEnter += MainForm_DragEnter; // �巡�� ���� �̺�Ʈ �ڵ鷯 ���
            this.DragDrop += MainForm_DragDrop; // ��� �̺�Ʈ �ڵ鷯 ���
            this.listView.View = View.Details;
            this.listView.HeaderStyle = ColumnHeaderStyle.None;
            this.listView.FullRowSelect = true;
            this.listView.Columns.Add("", -2);
            updateUI();
            setSizeContainer();
        }
        private bool Open(string targetPath)
        {
            setClose();
            //pictureBox.Image = null;
            compressedFile = CFM_CompressedFile.Open(targetPath);
            if (compressedFile == null)
                return false;
            lastFile = targetPath;
            textBox_FileName.Text = lastFile;
            //Regacy TreeView
            //treeView.Nodes.Clear();
            //GC.Collect();
            //TreeNode root = new TreeNode(Path.GetFileName(targetPath));
            //foreach (var file in compressedFile.FileList)
            //{
            //    root.Nodes.Add(file);
            //}
            //treeView.Nodes.Add(root);
            //root.Toggle();

            //GC.Collect();
            foreach (var file in compressedFile.FileList)
            {
                listView.Items.Add(file);
            }
            updateUI();
            return true;
        }
        private void setClose()
        {
            compressedFile = null;
            //Regacy TreeView
            //treeView.Nodes.Clear();
            listView.Items.Clear();
            textBox_FileName.Text = "";
            setClearTempImage();
            GC.Collect();

            updateUI();
        }
        private void setClearTempImage()
        {
            lastSelectIndex = -1;
            pictureBox.Image = null;
            currentImage = "";
            textBox_TempFileName.Text = "";

        }

        private bool isOpen { get { return compressedFile != null; } }

        private void updateUI()
        {
            bool bOpened = isOpen;
            if (!bOpened)
            {
                listView.Items.Clear();
                setClearTempImage();
            }
            button_Close.Enabled = bOpened;
            button_Recompress.Enabled = bOpened;
            button_RecompressAs.Enabled = bOpened;
            button_Delete.Enabled = bOpened;
            button_RevertDelete.Enabled = bOpened;
            //Regacy TreeView
            //treeView.Enabled = bOpened;
            listView.Enabled = bOpened;
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
                if (currentImage == path)
                    return;
                try
                {
                    if (Path.GetExtension(path) == ".webp")
                    {
                        // WebP ������ Image<Rgba32> ��ü�� �ε�
                        using (FileStream fileStream = File.OpenRead(path))
                        {
                            SixLabors.ImageSharp.Image image = SixLabors.ImageSharp.Image.Load(path);
                            using (MemoryStream ms = new MemoryStream())
                            {
                                image.Save(ms, PngFormat.Instance);
                                pictureBox.Image = System.Drawing.Image.FromStream(ms);
                            }
                        }
                    }
                    else
                    {
                        pictureBox.Image = System.Drawing.Image.FromFile(path);
                    }
                    currentImage = path;

                    textBox_TempFileName.Text = Path.GetFileName(path);

                    pictureBox.SizeMode = PictureBoxSizeMode.Zoom;
                    pictureBox.Dock = DockStyle.Fill;
                }
                catch
                {
                    MessageBox.Show("Image Load Fail!");
                }

            }, path
            );

            return true;
        }
        private void setSizeContainer()
        {
            //Regacy TreeView
            //sizeContainer.Main_Top=treeView.Top;
            //sizeContainer.Main_BottomOffset = this.Height -treeView.Bottom;
            sizeContainer.Main_Top = listView.Top;
            sizeContainer.Main_BottomOffset = this.Height - listView.Bottom;
            sizeContainer.Bottom_TopOffset = this.Height - panel_Button.Top;
            sizeContainer.Main_RightOffset = this.Width - panel.Right;

        }
        private void setControlSize()
        {
            panel_Button.Top = this.Height - sizeContainer.Bottom_TopOffset;

            //Regacy TreeView
            //treeView.Top = sizeContainer.Main_Top;
            listView.Top = sizeContainer.Main_Top;
            panel.Top = sizeContainer.Main_Top;

            int mainHeight = this.Height - sizeContainer.Main_BottomOffset - sizeContainer.Main_Top;
            //Regacy TreeView
            //treeView.Height= mainHeight;
            //panel.Left = treeView.Right + 10;
            listView.Height = mainHeight;
            panel.Left = listView.Right + 10;

            panel.Width = this.Width - sizeContainer.Main_RightOffset - panel.Left;
            panel.Height = mainHeight;

            textBox_TempFileName.Left = panel.Left;
            textBox_TempFileName.Width = this.Width - sizeContainer.Main_RightOffset - panel.Left;

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

        //Regacy TreeView
        //private void treeView_AfterSelect(object sender, TreeViewEventArgs e)
        //{
        //    if (treeView.SelectedNode == null)
        //        return;
        //    if (e.Node == null)
        //        return;
        //    if (e.Node == treeView.Nodes[0])
        //        return;
        //    selectItem(e.Node.Index);
        //}
        private void listView_SelectedIndexChanged(object sender, EventArgs e)
        {

            if (listView.SelectedItems.Count <= 0)
            {
                setClearTempImage();
                return;
            }
            if (listView.SelectedItems[0] == null)
            {
                setClearTempImage();
                return;
            }
            if (lastSelectIndex != listView.SelectedIndices[0])
            {
                lastSelectIndex = listView.SelectedIndices[0];
                selectItem(listView.SelectedIndices[0]);
            }
        }

        private void button_Exit_Click(object sender, EventArgs e)
        {
            Application.Exit();
        }

        private void button_Close_Click(object sender, EventArgs e)
        {
            setClose();

        }

        private void button_Recompress_Click(object sender, EventArgs e)
        {
            if (compressedFile == null)
                return;

            if (compressedFile.Recompress())
            {
                MessageBox.Show("Recompress Success!");
                setClose();
                Open(lastFile);
            }
            else
            {
                MessageBox.Show("Recompress Fail!");

            }
        }

        private void button_RecompressAs_Click(object sender, EventArgs e)
        {
            if (compressedFile == null)
                return;

            // SaveFileDialog ��ü ����
            SaveFileDialog saveFileDialog1 = new SaveFileDialog();

            // ���� ���� ���� (��: PNG �̹���)
            saveFileDialog1.Filter = "Zip Image|*.zip";

            // ��ȭ ���ڸ� ���� ����ڰ� ������ �����ϸ�
            if (saveFileDialog1.ShowDialog() == DialogResult.OK)
            {
                // ������ ���� ��� ��������
                string filePath = saveFileDialog1.FileName;
                if (compressedFile.Recompress(filePath))
                {
                    MessageBox.Show("Recompress Success!");
                    setClose();
                    Open(filePath);
                }
                else
                {
                    MessageBox.Show("Recompress Fail!");

                }

            }
        }

        private void button_Delete_Click(object sender, EventArgs e)
        {
            if (compressedFile == null)
                return;
            //Regacy TreeView
            //if (treeView.SelectedNode == null)
            //    return;
            //var node = treeView.SelectedNode;
            //if (node == treeView.Nodes[0])
            //    return;
            //if (node.ForeColor == System.Drawing.Color.WhiteSmoke)
            //    return;
            //node.ForeColor = System.Drawing.Color.WhiteSmoke;
            //if (!compressedFile.DeleteFile(node.Index))
            //{
            //    MessageBox.Show("Delete Fail!");

            //}
            if (listView.SelectedIndices.Count <= 0)
                return;
            foreach (int index in listView.SelectedIndices)
            {
                var item = listView.Items[index];
                if (item.ForeColor == System.Drawing.Color.WhiteSmoke)
                    continue;
                item.ForeColor = System.Drawing.Color.WhiteSmoke;
                if (!compressedFile.DeleteFile(index))
                {
                    MessageBox.Show("Delete Fail!");
                    break;
                }
            }
            //listView.Focus();
            listView.SelectedIndices.Clear();
            listView.Refresh();
        }

        private void button_RevertDelete_Click(object sender, EventArgs e)
        {
            if (compressedFile == null)
                return;
            //Regacy TreeView
            //if (treeView.SelectedNode == null)
            //    return;
            //var node = treeView.SelectedNode;
            //if (node == treeView.Nodes[0])
            //    return;
            //if (node.ForeColor != System.Drawing.Color.WhiteSmoke)
            //    return;
            //node.ForeColor = System.Drawing.Color.Black;
            //if (!compressedFile.RevertDeleteFile(node.Index))
            //{
            //    MessageBox.Show("RevertDeleteFile Fail!");
            //
            //}
            if (listView.SelectedIndices.Count <= 0)
                return;
            foreach (int index in listView.SelectedIndices)
            {
                var item = listView.Items[index];
                if (item.ForeColor != System.Drawing.Color.WhiteSmoke)
                    continue;
                item.ForeColor = System.Drawing.Color.Black;
                if (!compressedFile.RevertDeleteFile(index))
                {
                    MessageBox.Show("RevertDeleteFile Fail!");
                    break;
                }
            }
            listView.SelectedIndices.Clear();
            listView.Refresh();
        }

        private void listView_KeyDown(object sender, KeyEventArgs e)
        {
            if (compressedFile == null)
                return;
            //Regacy TreeView
            //if (treeView.SelectedNode == null)
            //    return;
            //if (e.KeyCode == Keys.Delete)
            //{
            //    var node = treeView.SelectedNode;
            //    if (node == treeView.Nodes[0])
            //        return;
            //    if (node.ForeColor == System.Drawing.Color.WhiteSmoke)
            //        return;
            //    node.ForeColor = System.Drawing.Color.WhiteSmoke;
            //    if (!compressedFile.DeleteFile(node.Index))
            //    {
            //        MessageBox.Show("Delete Fail!");
            //    }
            //}
            if (listView.SelectedIndices.Count <= 0)
                return;
            if (e.KeyCode == Keys.Delete)
            {
                foreach (int index in listView.SelectedIndices)
                {
                    var item = listView.Items[index];
                    if (item.ForeColor != System.Drawing.Color.WhiteSmoke)
                        continue;
                    item.ForeColor = System.Drawing.Color.Black;
                    if (!compressedFile.RevertDeleteFile(index))
                    {
                        MessageBox.Show("RevertDeleteFile Fail!");
                        break;
                    }
                }
            }
        }

        private void MainForm_Resize(object sender, EventArgs e)
        {
            setControlSize();
        }
    }
}
