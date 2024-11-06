use clap::{command, Arg};
use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::process::{Command, ExitStatus};

fn matches() -> clap::ArgMatches {
    return command!()
        .name("PyIDE Python Project setup")
        .version("1.0.0")
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
            Arg::new("vscode")
                .short('c')
                .long("code")
                .help("Open the project in Visual Studio Code as soon as it is created")
                .required(false)
                .num_args(0),
        )
        .arg(
            Arg::new("pycharm")
                .short('p')
                .long("pycharm")
                .help("Open the project in PyCharm as soon as it is created")
                .required(false)
                .num_args(0),
        )
        .arg(
            Arg::new("zed")
                .short('z')
                .long("zed")
                .help("Open the project in Zed as soon as it is created")
                .required(false)
                .num_args(0),
        )
        .get_matches();
}

fn upgrade_pip(project_name: &str) -> Result<(), Box<dyn Error>> {
    let upgrade_pip: ExitStatus = Command::new(format!("{}/.venv/bin/pip3", project_name))
        .arg("install")
        .arg("--upgrade")
        .arg("pip")
        .spawn()?
        .wait()?;

    if !upgrade_pip.success() {
        return Err(Box::from("Failed to upgrade pip"));
    }

    Ok(())
}

fn install_modules(project_name: &str, modules: Vec<String>) -> Result<(), Box<dyn Error>> {
    let install_modules: ExitStatus = Command::new(format!("{}/.venv/bin/pip3", project_name))
        .arg("install")
        .args(&modules)
        .spawn()?
        .wait()?;

    if !install_modules.success() {
        return Err(Box::from("Failed to install modules"));
    }

    Ok(())
}

fn create_project(
    project_name: &str,
    modules: Vec<String>,
    ide: &str,
) -> Result<(), Box<dyn Error>> {
    // create project directory
    fs::create_dir(project_name)?;

    // create virtual environment
    let venv: ExitStatus = Command::new("python3")
        .arg("-m")
        .arg("venv")
        .arg(format!("{}/.venv", project_name))
        .spawn()?
        .wait()?;

    if !venv.success() {
        return Err(Box::from("Failed to create virtual environment"));
    }

    println!(
        "Python3 Virtual environment created in {}/.venv",
        project_name
    );

    // upgrade pip
    upgrade_pip(project_name)?;

    // install modules
    install_modules(project_name, modules)?;

    // open project in IDE
    match ide {
        "vscode" => {
            Command::new("code").arg(project_name).spawn()?;
        }
        "pycharm" => {
            Command::new("charm").arg(project_name).spawn()?;
        }
        "zed" => {
            Command::new("zeditor").arg(project_name).spawn()?;
        }
        _ => {}
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches: clap::ArgMatches = matches();

    let project_name: String = matches.get_one::<String>("name").unwrap().clone();

    let ide: &str = ["vscode", "pycharm", "zed"]
        .iter()
        .find(|&&ide| matches.value_source(ide) == Some(clap::parser::ValueSource::CommandLine))
        .unwrap_or(&"");

    let modules: Vec<String> = matches
        .get_many::<String>("modules")
        .map(|vals| vals.cloned().collect())
        .unwrap_or_default();

    // remove duplicates
    let modules: Vec<String> = modules
        .into_iter()
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    create_project(&project_name, modules, ide)?;
    Ok(())
}
