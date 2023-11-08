// use rfd::FileDialog;
// use std::{env::args, fs, os::unix::fs::symlink};
// fn main() {
//     let input_args: Vec<String> = args().collect();
//     println!("input_args {:#?}", input_args);
//     // let root_path = "/Users/wangyansong/project";

//     // let target_path = FileDialog::new()
//     //     .set_directory(root_path)
//     //     .pick_folder()
//     //     .expect("取消选择文件夹");
//     // println!("要链接的目标目录 {}", target_path.display());
//     // let link_path = FileDialog::new()
//     //     .set_directory(root_path)
//     //     .pick_folder()
//     //     .expect("取消选择文件夹");
//     // println!("被链接的目录 {}", link_path.display());

//     // fs::remove_dir_all(link_path.as_path()).unwrap();

//     // match symlink(target_path.as_path(), link_path.as_path()) {
//     //     Ok(_) => println!("符号链接创建成功！"),
//     //     Err(e) => println!("创建符号链接时出错: {:?}", e),
//     // }
// }

use std::process::exit;

use args::{Args, ArgsError};

extern crate args;
fn main() {
    match prase(std::env::args().collect()) {
        Ok(_) => println!("Successfully parsed args"),
        Err(error) => {
            println!("{}", error);
            exit(1);
        }
    };
}

fn prase(input: Vec<String>) -> Result<(), ArgsError> {
    let mut args = Args::new("rs_ln", "rs ln");
    args.flag("h", "help", "Print the usage menu");
    args.parse(input);
    let help = args.value_of("help")?;
    if help {
        args.full_usage();
        return Ok(());
    }

    return Ok(());
}
