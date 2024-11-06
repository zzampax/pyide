# PyIDE Python Project Templater

<br>
<div align="center">

![Language](https://img.shields.io/github/languages/top/zzampax/PyIDE.svg?style=for-the-badge&labelColor=black&logo=rust&logoColor=red&label=Rust)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)
![Github](https://img.shields.io/badge/GitHub-000000?style=for-the-badge&logo=github&logoColor=white)
![WAM](https://img.shields.io/badge/AUTOMATE-EVERYTHING-CD3713?style=for-the-badge&labelColor=black)

<img src=".github/logo.jpg" alt="RU:PYTHON" height="300px">
</div>
<br>

This is a simple Python project templater that creates a basic project structure for a Python project:
```bash
.
├── .git/
├── .venv/
├── .gitignore
└── main.py

2 directories, 2 files
```
The newly created project will have version control (`git`), a virtual environment, a `.gitignore` file in which the `.venv` directory is flagged, and a `main.py` file.
## Usage
```bash
$ ./pyide <project_name> -m <modules>
```
For more specific information:
```bash
$ ./pyide --help
```
Optionally you can specify the editor you want to use:
### Visual Studio Code
```bash
$ ./pyide <project_name> -m <modules> -c
```
### PyCharm
```bash
$ ./pyide <project_name> -m <modules> -p
```
### Zed
```bash
$ ./pyide <project_name> -m <modules> -z
```
## Compilation
```bash
$ cargo build --release
```
