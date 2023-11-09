use clap::Parser;
use rfd::FileDialog;
use std::{
    os::unix::fs::symlink,
    path::{self, PathBuf}, fs,
};
#[derive(Parser, Debug)]
#[command(
    author = "wyswill",
    version = "0.0.1",
    about = "rs_ln",
    long_about = "ln on rust"
)]
struct Args {
    #[arg(short, long, help = "指定源目录")]
    source: String,

    #[arg(short, long, help = "指定目标目录")]
    target: String,

    #[arg(short, long, help = "使用弹窗选择目录", default_value_t = false)]
    watch_mode: bool,
}

fn main() {
    let args = Args::parse();
    let sp = path::Path::new(&args.source);
    let tp = path::Path::new(&args.target);
    if !sp.exists() {
        panic!("源路径错误!");
    }
    if !tp.exists() {
        panic!("目标路径错误!");
    }

    if args.watch_mode {
        let target_path = FileDialog::new()
            .set_directory(args.source.clone())
            .pick_folder()
            .expect("取消选择文件夹");
        create_symlink(PathBuf::from(args.source.clone()), target_path);
    } else {
        create_symlink(
            PathBuf::from(args.source.clone()),
            PathBuf::from(args.target.clone()),
        );
    }
}

fn create_symlink(source_path: PathBuf, target_path: PathBuf) {
    fs::remove_dir_all(&source_path).unwrap();
    println!(
        "sp : {}, tp :{}",
        source_path.as_os_str().to_str().unwrap(),
        target_path.as_os_str().to_str().unwrap()
    );
    match symlink(source_path, target_path) {
        Ok(_) => println!("符号链接创建成功！"),
        Err(e) => println!("创建符号链接时出错: {:?}", e),
    }
}
