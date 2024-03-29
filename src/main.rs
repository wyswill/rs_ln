use clap::Parser;
use rfd::FileDialog;
use std::{
    env, fs,
    os::unix::fs::symlink,
    path::{self, PathBuf},
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
    let work_space = env::current_dir().expect("获取当前目录失败");
    println!("work_space {}", work_space.to_str().unwrap());

    let args = Args::parse();
    let sp = path::Path::new(&args.source);
    if !sp.exists() {
        panic!("源路径错误!");
    }

    if args.watch_mode {
        let target_path = FileDialog::new()
            .set_directory(work_space)
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
    if target_path.exists() {
        fs::remove_dir_all(&target_path).unwrap();
    }
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
