use rfd::FileDialog;
use std::{fs, os::unix::fs::symlink};
fn main() {
    let root_path = "/Users/wangyansong/project";

    let target_path = FileDialog::new()
        .set_directory(root_path)
        .pick_folder()
        .expect("取消选择文件夹");
    println!("要链接的目标目录 {}", target_path.display());
    let link_path = FileDialog::new()
        .set_directory(root_path)
        .pick_folder()
        .expect("取消选择文件夹");
    println!("被链接的目录 {}", link_path.display());

    fs::remove_dir(link_path.as_path()).unwrap();

    match symlink(target_path.as_path(), link_path.as_path()) {
        Ok(_) => println!("符号链接创建成功！"),
        Err(e) => println!("创建符号链接时出错: {:?}", e),
    }
}
