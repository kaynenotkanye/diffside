# diffside

> 🧠 A side-by-side CLI diff tool with word-level coloring, Dracula theme, and scrollable paging.

**diffside** is a command-line utility written in Rust that compares two text files and shows their differences in a clean, side-by-side format. It highlights word-level changes with background colors and includes paging support for viewing large files easily.

---

## ✨ Features

- 🟥🟩 **Word-level diff highlighting**
- 📜 **Side-by-side comparison with aligned line numbers**
- 🎨 **Built-in Dracula theme (no config needed)**
- 📏 **Automatic line wrapping and alignment**
- 🔽 **Paging support using `less -R`**
- ✅ **"No differences found" detection**

---

## 🚀 Installation

Once published to crates.io:

```bash
cargo install diffside
```

---

## 🧪 Example

Compare two files:

```bash
diffside file1.txt file2.txt
```

Disable pager (for piping or scripting):

```bash
diffside file1.txt file2.txt --no-pager
```

---

## ✅ No Diffs Case

When the files are identical, you'll see:

```
✅ No differences found between 'file1.txt' and 'file2.txt'.
```

---

## 📄 License

MIT © 2025 Kayne Amornvivat
