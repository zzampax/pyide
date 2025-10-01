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
Optionally you can specify the Python package and project manager you want to use:
### `uv-python`
```bash
$ ./pyide <project_name> -m <modules> -u
```
### `pip`
```bash
$ ./pyide <project_name> -m <modules> -p
```
It's also possible to specify the editor you'd like to use:
```bash
$ ./pyide <project_name> -m <modules> -i <editor>
```
Where `<editor>` can be one of the following:
- `vscode`
- `pycharm`
- `zed`
## Compilation
```bash
$ cargo build --release
```
