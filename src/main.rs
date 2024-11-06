use clap::{command, Arg};
use std::collections::HashSet;
use std::error::Error;
use std::fs;

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

fn create_project(project_name: &str, modules: Vec<String>) -> Result<(), Box<dyn Error>> {
    // create project directory
    fs::create_dir(project_name)?;

    // create virtual environment
    let venv = std::process::Command::new("python3")
        .arg("-m")
        .arg("venv")
        .arg(format!("{}/.venv", project_name))
        .output()?;

    if !venv.status.success() {
        return Err(Box::from("Failed to create virtual environment"));
    }

    println!(
        "Python3 Virtual environment created in {}/.venv",
        project_name
    );

    // install modules
    let install_modules = std::process::Command::new(format!("{}/.venv/bin/pip3", project_name))
        .arg("install")
        .args(&modules)
        .output()?;

    println!("Installed modules: {:?}", &modules);

    if !install_modules.status.success() {
        return Err(Box::from("Failed to install modules"));
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches: clap::ArgMatches = matches();

    let project_name: String = matches.get_one::<String>("name").unwrap().clone();

    let ide: String =
        if matches.value_source("vscode") == Some(clap::parser::ValueSource::CommandLine) {
            "vscode".to_string()
        } else if matches.value_source("pycharm") == Some(clap::parser::ValueSource::CommandLine) {
            "pycharm".to_string()
        } else if matches.value_source("zed") == Some(clap::parser::ValueSource::CommandLine) {
            "zed".to_string()
        } else {
            "".to_string()
        };

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

    println!("IDE: {}", ide);
    create_project(&project_name, modules)?;
    Ok(())
}
