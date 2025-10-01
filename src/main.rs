use clap::{command, Arg};
use std::collections::HashSet;
use std::error::Error;
use std::fs::{self, File};
use std::io::{BufRead, Write};
use std::process::{Command, ExitStatus};

fn matches() -> clap::ArgMatches {
    return command!()
        .name("PyIDE Python Project setup")
        .version("1.1.0")
        .about("Setup a Python IDE project")
        .arg(
            Arg::new("name")
                .help("Name of the Project")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("modules")
                .short('m')
                .long("modules")
                .help("Pip3 modules to install in the Project (separated by spaces) e.g. numpy pandas")
                .required(false)
                .num_args(1..),
        )
        .arg(
            Arg::new("uv-python")
                .short('u')
                .long("uv-python")
                .help("Use 'uv' to create the virtual environment and install packages. It'll use pip if uv is not installed (default)")
                .required(false)
                .num_args(0),
        )
        .arg(
            Arg::new("pip")
                .short('p')
                .long("pip")
                .help("Use 'pip' to install packages")
                .required(false)
                .num_args(0),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .help("Pip3 modules to install in the Project (separated by new lines) [OVERWRITES --modules]")
                .required(false)
                .num_args(1),
        )
        .arg(
            Arg::new("ide")
                .short('i')
                .long("ide")
                .help("IDE to open the project with as soon as it's created (vscode, pycharm, zed)")
                .required(false)
                .num_args(1),
        )
        .get_matches();
}

fn init_git(project_name: &str) -> Result<(), Box<dyn Error>> {
    let init_git: ExitStatus = Command::new("git")
        .arg("init")
        .arg(project_name)
        .spawn()?
        .wait()?;

    if !init_git.success() {
        return Err(Box::from("Failed to initialize git"));
    }

    // Create .gitignore file
    let gitignore: ExitStatus = Command::new("sh")
        .arg("-c")
        .arg(format!("echo '.venv' > {}/.gitignore", project_name))
        .spawn()?
        .wait()?;

    if !gitignore.success() {
        return Err(Box::from("Failed to create .gitignore file"));
    }

    println!("Git repository initialized in {}", project_name);

    Ok(())
}

fn create_venv(project_name: &str, check_venv: &str) -> Result<(), Box<dyn Error>> {
    match check_venv {
        "uv-python" => Command::new("uv")
            .arg("venv")
            .arg(format!("{}/.venv", project_name))
            .spawn()?
            .wait()?,
        "pip" => Command::new("python3")
            .arg("-m")
            .arg("venv")
            .arg(format!("{}/.venv", project_name))
            .spawn()?
            .wait()?,
        _ => return Err(Box::from("Failed to create virtual environment")),
    };

    println!(
        "Virtual environment created in {}/.venv",
        project_name
    );

    Ok(())
}

fn upgrade_pip(project_name: &str, check_venv: &str) -> Result<(), Box<dyn Error>> {
    match check_venv {
        "uv-python" => Command::new("uv")
            .args(["pip", "--directory", &format!("{}/.venv", project_name), "install", "--upgrade", "pip"])
            .spawn()?
            .wait()?,
        "pip" => Command::new(format!("{}/.venv/bin/pip3", project_name))
            .arg("install")
            .arg("--upgrade")
            .arg("pip")
            .spawn()?
            .wait()?,
        _ => return Err(Box::from("Failed to upgrade pip")),
    };

    Ok(())
}

fn install_modules(project_name: &str, modules: Vec<String>, check_venv: &str) -> Result<(), Box<dyn Error>> {
    if modules.is_empty() {
        return Ok(());
    }

    match check_venv {
        "uv-python" => Command::new("uv")
            .args(["pip", "--directory", &format!("{}/.venv", project_name), "install"])
            .args(&modules)
            .spawn()?
            .wait()?,
        "pip" => Command::new(format!("{}/.venv/bin/pip3", project_name))
            .arg("install")
            .args(&modules)
            .spawn()?
            .wait()?,
        _ => return Err(Box::from("Failed to install modules")),
    };

    Ok(())
}

fn check_ide(ide: &str) -> Result<(), Box<dyn Error>> {
    let cmd_str: &str = match ide {
        "vscode" => "code",
        "pycharm" => "charm",
        "zed" => "zeditor",
        _ => return Err(Box::from("No IDE selected")),
    };

    let check_ide: ExitStatus = match Command::new(cmd_str).arg("--version").spawn() {
        Ok(mut cmd) => cmd.wait()?,
        Err(_) => return Err(Box::from("IDE not installed")),
    };

    if !check_ide.success() {
        return Err(Box::from("IDE not installed"));
    }

    Ok(())
}

fn create_main(project_name: &str) -> Result<(), Box<dyn Error>> {
    let mut main_file = File::create(format!("{}/main.py", project_name))?;
    main_file.write_all(b"print('Hello, World!')")?;

    Ok(())
}

fn open_ide(project_name: &str, ide: &str) -> Result<(), Box<dyn Error>> {
    // check if IDE is installed
    check_ide(ide)?;

    println!("Opening project in {}", ide);

    let open_ide: ExitStatus = match ide {
        "vscode" => Command::new("code")
            .arg("-n")
            .arg(project_name)
            .spawn()?
            .wait()?,
        "pycharm" => Command::new("charm")
            .arg("-n")
            .arg(project_name)
            .spawn()?
            .wait()?,
        "zed" => Command::new("zeditor")
            .arg("-n")
            .arg(project_name)
            .spawn()?
            .wait()?,
        _ => return Ok(()),
    };

    if !open_ide.success() {
        return Err(Box::from("Failed to open IDE"));
    }

    Ok(())
}

fn create_project(
    project_name: &str,
    modules: Vec<String>,
    ide: &str,
    venv: &str,
) -> Result<(), Box<dyn Error>> {
    // check if project already exists
    if fs::metadata(project_name).is_ok() {
        return Err(Box::from("Project already exists"));
    }

    // create project directory
    fs::create_dir(project_name)?;

    // initialize git
    init_git(project_name)?;

    // create virtual environment
    create_venv(project_name, &venv)?;

    // upgrade pip
    upgrade_pip(project_name, &venv)?;

    // install modules
    install_modules(project_name, modules, &venv)?;

    // create main.py
    create_main(project_name)?;

    // open project in IDE
    open_ide(project_name, ide)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches: clap::ArgMatches = matches();

    let project_name: String = matches.get_one::<String>("name").unwrap().clone();

    let mut modules: Vec<String>;
    modules = matches
        .get_many::<String>("modules")
        .map(|vals| vals.cloned().collect())
        .unwrap_or_default();

    let ide: &str = matches
        .get_one::<String>("ide")
        .map(|s| s.as_str())
        .unwrap_or("");

    let venv: &str = ["uv-python", "pip"]
        .iter()
        .find(|&&venv| matches.value_source(venv) == Some(clap::parser::ValueSource::CommandLine))
        .unwrap_or(&"uv-python");

    let file: String = matches
        .get_one::<String>("file")
        .unwrap_or(&"".to_string())
        .clone();

    if !file.is_empty() {
        let file = File::open(&file)?;
        let reader = std::io::BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            modules.push(line);
        }
    }

    // remove duplicates
    let modules: Vec<String> = modules
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    create_project(&project_name, modules, ide, venv)?;
    Ok(())
}
