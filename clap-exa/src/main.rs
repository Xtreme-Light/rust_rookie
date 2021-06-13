use ansi_term::Colour::{Cyan, Green, Red};
use clap::{App, Arg};
use std::ffi::OsString;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::FileTypeExt;
use std::path::PathBuf;
use std::{env, fs};

///
///通过clap构建命令行程序,有多种方式,你可以通过阅读
/// [quick_example](https://github.com/clap-rs/clap/blob/master/examples/01a_quick_example.rs)
/// [quick_example](https://github.com/clap-rs/clap/blob/master/examples/01b_quick_example.rs)
/// [quick_example](https://github.com/clap-rs/clap/blob/master/examples/01c_quick_example.rs)
/// 来选择你喜欢的方式来构建程序
///
fn main() {
    let matches = App::new("clap-exa Program")
        .version("0.0.1")
        .author("extreme light")
        .about("learn how to build a exa program by clap")
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .about("show hidden and 'dot' files")
                .takes_value(false),
        )
        .get_matches();
    if matches.is_present("all") {
        let cur_dir = PathBuf::from("/home/light/RustProjects/rust_rookie");
        let std_out = &mut std::io::stdout();
        for entry in fs::read_dir(cur_dir).unwrap() {
            let entry = entry.unwrap();
            let result = fs::metadata(entry.path()).unwrap();
            let file_type = result.file_type();
            if file_type.is_dir() {
                let mut dir_name = entry.file_name();
                dir_name.push("  ");
                Cyan.paint(dir_name.as_bytes()).write_to(std_out).unwrap();
            }
            if file_type.is_file() {
                let mut file_name = entry.file_name();
                file_name.push("  ");
                Red.paint(file_name.as_bytes()).write_to(std_out).unwrap();
            }
        }
    } else {
        let cur_dir = PathBuf::from("/home/light/RustProjects/rust_rookie");
        let std_out = &mut std::io::stdout();
        for entry in fs::read_dir(cur_dir).unwrap() {
            let entry = entry.unwrap();
            if !entry
                .file_name()
                .to_str()
                .unwrap()
                .as_bytes()
                .starts_with(".".as_ref())
            {
                let result = fs::metadata(entry.path()).unwrap();
                let file_type = result.file_type();
                if file_type.is_dir() {
                    let mut dir_name = entry.file_name();
                    dir_name.push("  ");
                    Cyan.paint(dir_name.as_bytes()).write_to(std_out).unwrap();
                }
                if file_type.is_file() {
                    let mut file_name = entry.file_name();
                    file_name.push("  ");
                    Red.paint(file_name.as_bytes()).write_to(std_out).unwrap();
                }
            }
        }
    }
    // let cur_dir = env::current_dir().unwrap();
}
