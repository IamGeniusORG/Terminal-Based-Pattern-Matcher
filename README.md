# 🚀 Terminal-Based Pattern Matcher (A Blazing Fast `grep` Clone)

Welcome to **Terminal-Based Pattern Matcher**! This is a lightning-fast, highly optimized command-line tool built entirely in **Rust**. 

Think of it as a modern, memory-safe version of the classic `grep` command. You can use it to instantly search through thousands of files or massive gigabyte-sized logs to find specific words, phrases, or patterns—without crashing your computer or eating up all your RAM.

## ✨ Why this tool is special:
- **Built for Speed**: Uses `rayon` to split the search across **all of your CPU cores** at the same time.
- **Memory Safe & Lightweight**: It reads files line-by-line using buffered I/O. Whether you search a 1 MB file or a 100 GB server log, the memory usage stays tiny.
- **Smart Regex Searching**: You aren't just limited to searching for exact words. You can search for advanced patterns like phone numbers, emails, or specific code structures.
- **Beautiful Output**: It doesn't just spit out boring text. It formats the output perfectly, highlighting the exact file path in magenta, the line number in cyan, and the matched word in bold green.
- **Folder Scanning**: Point it at an entire directory, and it will recursively dig through every single folder and file for you.

---

## 🛠️ How to Install It

1. First, make sure you have [Rust](https://rustup.rs/) installed on your computer.
2. Clone this repository:
   ```bash
   git clone https://github.com/IamGeniusORG/Terminal-Based-Pattern-Matcher.git
   cd "Terminal-Based-Pattern-Matcher"
   ```
3. Build the highly-optimized release version:
   ```bash
   cargo build --release
   ```
4. You will find the final, blazing-fast executable inside the `target/release/` folder. You can move `pattern_matcher.exe` anywhere on your computer to use it globally!

---

## 📖 Simple Examples (How to use it)

Here are a few real-world examples of how this tool can make your life easier:

### Example 1: Finding a specific word in a massive log file
Let's say you have a massive server log file and you want to find every time an "error" happened.
```bash
pattern_matcher "error" "server_logs.txt"
```
*(The tool will instantly print out every single line containing "error", along with the exact line number.)*

### Example 2: Searching an entire project folder
Let's say you forgot where you wrote a specific function called `calculate_tax` in your massive coding project. Instead of opening every file, just point the tool at your project folder (`src/`):
```bash
pattern_matcher "calculate_tax" "src/"
```
*(It will scan every single file inside the `src` folder at the same time and tell you exactly which file and line it is in!)*

### Example 3: Ignoring Uppercase vs Lowercase
If you are looking for the word "Password" but you aren't sure if it was typed as "PASSWORD", "Password", or "password", just add the `-i` flag (ignore case):
```bash
pattern_matcher "password" "config.txt" -i
```

### Example 4: Advanced Pattern Searching (Regex)
Let's say you have a massive document and you want to extract every single word that starts with "admin_". You can use a regex pattern:
```bash
pattern_matcher "admin_[a-z]+" "database_dump.txt"
```

---

## 👨‍💻 Built By
Developed to master modern systems-level programming, memory safety, and multi-threading in Rust.
