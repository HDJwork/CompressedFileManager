namespace CompressedFileManager
{
    partial class MainForm
    {
        /// <summary>
        ///  Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        ///  Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        ///  Required method for Designer support - do not modify
        ///  the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            //TreeView treeView;
            pictureBox = new PictureBox();
            button_Open = new Button();
            button_Recompress = new Button();
            button_Exit = new Button();
            button_Close = new Button();
            button_RecompressAs = new Button();
            treeView = new TreeView();
            ((System.ComponentModel.ISupportInitialize)pictureBox).BeginInit();
            SuspendLayout();
            // 
            // treeView
            // 
            treeView.Location = new Point(15, 12);
            treeView.Name = "treeView";
            treeView.Size = new Size(258, 377);
            treeView.TabIndex = 0;
            treeView.AfterSelect += treeView_AfterSelect;
            // 
            // pictureBox
            // 
            pictureBox.Location = new Point(286, 13);
            pictureBox.Name = "pictureBox";
            pictureBox.Size = new Size(494, 377);
            pictureBox.TabIndex = 1;
            pictureBox.TabStop = false;
            // 
            // button_Open
            // 
            button_Open.Location = new Point(17, 409);
            button_Open.Name = "button_Open";
            button_Open.Size = new Size(85, 31);
            button_Open.TabIndex = 2;
            button_Open.Text = "Open";
            button_Open.UseVisualStyleBackColor = true;
            button_Open.Click += button_Open_Click;
            // 
            // button_Recompress
            // 
            button_Recompress.Location = new Point(404, 409);
            button_Recompress.Name = "button_Recompress";
            button_Recompress.Size = new Size(106, 31);
            button_Recompress.TabIndex = 3;
            button_Recompress.Text = "Recompress";
            button_Recompress.UseVisualStyleBackColor = true;
            button_Recompress.Click += button_Recompress_Click;
            // 
            // button_Exit
            // 
            button_Exit.Location = new Point(682, 409);
            button_Exit.Name = "button_Exit";
            button_Exit.Size = new Size(106, 31);
            button_Exit.TabIndex = 4;
            button_Exit.Text = "Exit";
            button_Exit.UseVisualStyleBackColor = true;
            button_Exit.Click += button_Exit_Click;
            // 
            // button_Close
            // 
            button_Close.Location = new Point(188, 409);
            button_Close.Name = "button_Close";
            button_Close.Size = new Size(85, 31);
            button_Close.TabIndex = 5;
            button_Close.Text = "Close";
            button_Close.UseVisualStyleBackColor = true;
            button_Close.Click += button_Close_Click;
            // 
            // button_RecompressAs
            // 
            button_RecompressAs.Location = new Point(548, 409);
            button_RecompressAs.Name = "button_RecompressAs";
            button_RecompressAs.Size = new Size(106, 31);
            button_RecompressAs.TabIndex = 6;
            button_RecompressAs.Text = "Recompress As";
            button_RecompressAs.UseVisualStyleBackColor = true;
            button_RecompressAs.Click += button_RecompressAs_Click;
            // 
            // MainForm
            // 
            AutoScaleDimensions = new SizeF(7F, 15F);
            AutoScaleMode = AutoScaleMode.Font;
            ClientSize = new Size(800, 450);
            Controls.Add(button_RecompressAs);
            Controls.Add(button_Close);
            Controls.Add(button_Exit);
            Controls.Add(button_Recompress);
            Controls.Add(button_Open);
            Controls.Add(pictureBox);
            Controls.Add(treeView);
            Name = "MainForm";
            Text = "CompressedFileManager";
            ((System.ComponentModel.ISupportInitialize)pictureBox).EndInit();
            ResumeLayout(false);
        }

        #endregion

        private TreeView treeView;
        private PictureBox pictureBox;
        private Button button_Open;
        private Button button_Recompress;
        private Button button_Exit;
        private Button button_Close;
        private Button button_RecompressAs;
    }
}
