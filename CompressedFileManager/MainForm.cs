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
            this.AllowDrop = true; // 폼이 드래그 앤 드롭을 받을 수 있도록 설정
            this.DragEnter += MainForm_DragEnter; // 드래그 진입 이벤트 핸들러 등록
            this.DragDrop += MainForm_DragDrop; // 드롭 이벤트 핸들러 등록
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
                        // WebP 파일을 Image<Rgba32> 객체로 로드
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

            // SaveFileDialog 객체 생성
            SaveFileDialog saveFileDialog1 = new SaveFileDialog();

            // 파일 필터 설정 (예: PNG 이미지)
            saveFileDialog1.Filter = "Zip Image|*.zip";

            // 대화 상자를 열고 사용자가 파일을 선택하면
            if (saveFileDialog1.ShowDialog() == DialogResult.OK)
            {
                // 선택한 파일 경로 가져오기
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
