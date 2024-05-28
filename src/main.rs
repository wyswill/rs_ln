use clap::{value_parser, Arg, ArgAction, Command};
use rfd::FileDialog;
use std::{
    env::{self},
    fs, os,
    path::{self, PathBuf},
};

fn main() {
    let work_space = env::current_dir().expect("获取当前目录失败");
    println!("work_space {}", style(work_space.to_str().unwrap()).cyan());

    let matches = Command::new("rs_ln")
        .about("rs_ln")
        .author("wyswill")
        .long_about("ln tool in rust")
        .args([
            Arg::new("source")
                .short('s')
                .required(true)
                .value_parser(value_parser!(String))
                .action(ArgAction::Set)
                .help("source_dir"),
            Arg::new("target")
                .short('t')
                .required(true)
                .value_parser(value_parser!(String))
                .help("target_dir"),
            Arg::new("open")
                .short('o')
                .value_parser(value_parser!(bool))
                .required(false)
                .help("open a dialog"),
        ])
        .get_matches();
    let source_path = matches.get_one::<String>("source").unwrap();
    let target_path = matches.get_one::<String>("target").unwrap();
    let open_dialog = matches.try_get_one::<bool>("open").unwrap();
    println!("source {}  target  {} ", source_path, target_path);
    let sp = path::Path::new(source_path);
    if !sp.exists() {
        panic!("源路径错误!");
    }

    if let Some(_is_open) = open_dialog {
        println!(
            "source {}  target  {} open_dialog {}",
            source_path, target_path, _is_open
        );
        let target_path = FileDialog::new()
            .set_directory(work_space)
            .pick_folder()
            .expect("取消选择文件夹");
        create_symlink(PathBuf::from(source_path), target_path);
    } else {
        create_symlink(PathBuf::from(source_path), PathBuf::from(target_path));
    }
}

fn create_symlink(source_path: PathBuf, target_path: PathBuf) {
    if target_path.exists() {
        fs::remove_dir_all(&target_path).unwrap();
    }
    println!(
        "sp : {}, tp :{}",
        source_path.as_os_str().to_str().unwrap(),
        target_path.as_os_str().to_str().unwrap()
    );
    match std::env::consts::OS {
        "linux" => println!("This is Linux!"),
        "macos" => {}
        "windows" => match os::windows::fs::symlink_dir(source_path, target_path) {
            Ok(_) => println!("符号链接创建成功！"),
            Err(e) => println!("创建符号链接时出错: {:?}", e),
        },
        _ => println!("Unknown OS"),
    }
}
