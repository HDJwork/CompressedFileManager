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
            panel = new Panel();
            button_Delete = new Button();
            button_RevertDelete = new Button();
            panel_Button = new Panel();
            treeView = new TreeView();
            ((System.ComponentModel.ISupportInitialize)pictureBox).BeginInit();
            panel.SuspendLayout();
            panel_Button.SuspendLayout();
            SuspendLayout();
            // 
            // pictureBox
            // 
            pictureBox.Location = new Point(22, 26);
            pictureBox.Name = "pictureBox";
            pictureBox.Size = new Size(450, 328);
            pictureBox.TabIndex = 1;
            pictureBox.TabStop = false;
            // 
            // button_Open
            // 
            button_Open.Location = new Point(16, 5);
            button_Open.Name = "button_Open";
            button_Open.Size = new Size(60, 45);
            button_Open.TabIndex = 2;
            button_Open.Text = "Open";
            button_Open.UseVisualStyleBackColor = true;
            button_Open.Click += button_Open_Click;
            // 
            // button_Recompress
            // 
            button_Recompress.Location = new Point(392, 5);
            button_Recompress.Name = "button_Recompress";
            button_Recompress.Size = new Size(106, 45);
            button_Recompress.TabIndex = 3;
            button_Recompress.Text = "Recompress";
            button_Recompress.UseVisualStyleBackColor = true;
            button_Recompress.Click += button_Recompress_Click;
            // 
            // button_Exit
            // 
            button_Exit.Location = new Point(670, 5);
            button_Exit.Name = "button_Exit";
            button_Exit.Size = new Size(106, 45);
            button_Exit.TabIndex = 4;
            button_Exit.Text = "Exit";
            button_Exit.UseVisualStyleBackColor = true;
            button_Exit.Click += button_Exit_Click;
            // 
            // button_Close
            // 
            button_Close.Location = new Point(212, 5);
            button_Close.Name = "button_Close";
            button_Close.Size = new Size(60, 45);
            button_Close.TabIndex = 5;
            button_Close.Text = "Close";
            button_Close.UseVisualStyleBackColor = true;
            button_Close.Click += button_Close_Click;
            // 
            // button_RecompressAs
            // 
            button_RecompressAs.Location = new Point(536, 5);
            button_RecompressAs.Name = "button_RecompressAs";
            button_RecompressAs.Size = new Size(106, 45);
            button_RecompressAs.TabIndex = 6;
            button_RecompressAs.Text = "Recompress As";
            button_RecompressAs.UseVisualStyleBackColor = true;
            button_RecompressAs.Click += button_RecompressAs_Click;
            // 
            // panel
            // 
            panel.Controls.Add(pictureBox);
            panel.Location = new Point(279, 12);
            panel.Name = "panel";
            panel.Size = new Size(500, 377);
            panel.TabIndex = 7;
            // 
            // button_Delete
            // 
            button_Delete.Location = new Point(82, 5);
            button_Delete.Name = "button_Delete";
            button_Delete.Size = new Size(60, 45);
            button_Delete.TabIndex = 8;
            button_Delete.Text = "Delete";
            button_Delete.UseVisualStyleBackColor = true;
            button_Delete.Click += button_Delete_Click;
            // 
            // button_RevertDelete
            // 
            button_RevertDelete.Location = new Point(146, 5);
            button_RevertDelete.Name = "button_RevertDelete";
            button_RevertDelete.Size = new Size(60, 45);
            button_RevertDelete.TabIndex = 9;
            button_RevertDelete.Text = "Revert Delete";
            button_RevertDelete.UseVisualStyleBackColor = true;
            button_RevertDelete.Click += button_RevertDelete_Click;
            // 
            // treeView
            // 
            treeView.Location = new Point(15, 12);
            treeView.Name = "treeView";
            treeView.Size = new Size(258, 377);
            treeView.TabIndex = 0;
            treeView.AfterSelect += treeView_AfterSelect;
            treeView.KeyDown += treeView_KeyDown;
            // 
            // panel_Button
            // 
            panel_Button.Controls.Add(button_RevertDelete);
            panel_Button.Controls.Add(button_Exit);
            panel_Button.Controls.Add(button_Delete);
            panel_Button.Controls.Add(button_Open);
            panel_Button.Controls.Add(button_Recompress);
            panel_Button.Controls.Add(button_RecompressAs);
            panel_Button.Controls.Add(button_Close);
            panel_Button.Location = new Point(1, 390);
            panel_Button.Name = "panel_Button";
            panel_Button.Size = new Size(797, 60);
            panel_Button.TabIndex = 8;
            // 
            // MainForm
            // 
            AutoScaleDimensions = new SizeF(7F, 15F);
            AutoScaleMode = AutoScaleMode.Font;
            ClientSize = new Size(800, 450);
            Controls.Add(panel);
            Controls.Add(treeView);
            Controls.Add(panel_Button);
            Name = "MainForm";
            Text = "CompressedFileManager";
            Resize += MainForm_Resize;
            ((System.ComponentModel.ISupportInitialize)pictureBox).EndInit();
            panel.ResumeLayout(false);
            panel_Button.ResumeLayout(false);
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
        private Panel panel;
        private Button button_Delete;
        private Button button_RevertDelete;
        private Panel panel_Button;
    }
}
