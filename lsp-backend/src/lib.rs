#![allow (dead_code, unused_imports, unused_variables)]

mod stack;
mod messenger;
mod vfs_loop;

pub use camino::{Utf8Component, Utf8Components, Utf8Path, Utf8PathBuf, Utf8Prefix};
use lsp_server::Message;
use core::error;
use std::{error::Error, env, fs::{self, File}, path::{Path, PathBuf}, io::{self, Write}};
use indexmap::{IndexMap, IndexSet};
use std::hash::BuildHasherDefault;
use rustc_hash::FxHasher; 
use stack::{MyStack, Stack};
use messenger::Messenger;
use std::process::Command;
use vfs_loop::vfs_loop;

pub struct VirtualFile {
    file_name: String,
    path: PathBuf,
    content: String,
}

pub struct VFS {
    pub files: IndexMap<u32, VirtualFile>
}

//@todo
pub enum Change {
    Create,
    Modify,
    Delete
}

pub struct History {
    pub history_stack: MyStack<(VirtualFile, Change)>,
}

/*
@todo
history
undo
命令行接口rustc
 */
impl Change {

    pub fn to_string (&self) -> String {
        match self {
            Change::Create => "Change".to_string(),
            Change::Modify => "Modify".to_string(),
            Change::Delete => "Delete".to_string()
        }
    }
}

/*
@todo
 */
pub trait Copy {
    fn copy (&self) {

    }
}

impl VirtualFile {

    pub fn from (file_name: String) -> VirtualFile {
        //let fileid: u32 = 1;
        let mut path = env::current_dir().unwrap();
        path.push(file_name.clone());
        let contents = fs::read_to_string(file_name.clone()).unwrap();

        VirtualFile { file_name, path, content: contents }
    }

    /*
    按行搜索文件内容
     */
    pub fn search<'a> (query: &str, vf: &'a VirtualFile) -> Vec<&'a str> {
        let mut results = Vec::new();
    
        for line in vf.content.lines() {
            if line.contains(query) {
                results.push(line);
            }
        }
    
        results
    }

    /*
    VF内容更新
     */
    pub fn update (&mut self) -> Result<(), Box<dyn std::error::Error>>{
        self.content = fs::read_to_string(self.file_name.clone()).unwrap();
        let mut file_handle = File::create(self.path.clone())?;
        //println!("{}", path.clone().to_str().unwrap());
        write!(file_handle, "{}", self.content)?;
        Ok(())
    }

}

impl VFS {

    pub fn start_vfs () {

        let messenger: (lsp_server::Connection, lsp_server::IoThreads) = Messenger::new();
        let vfs = VFS::new();
        //match vfs_loop(messenger.0, vfs) {}

        

    }


    pub fn new () -> VFS {

        let handle: IndexMap<u32, VirtualFile> = IndexMap::new();
        VFS { files: handle }
    }

    /*
    使用文件名将文件加入vfs
     */
    pub fn append (mut self, filename: String) -> VFS {
        let vf = VirtualFile::from(filename);
        //let change = Change::Create;
        self.files.insert(1, vf);
        self
    }

    /*
    使用文件id获取内容,hash
     */
    pub fn get_content_from_file_id (&self, file_id: u32) -> Option<String> {
        let x = self.files.get(&file_id);

        match x {
            None => None,
            Some(i) => Some(i.content.clone()),
        }
    }
    
    /*
    使用文件名获取内容,遍历
     */
    pub fn get_content_from_file_name (&self, file_name: String) -> Option<String> {
        for vf in &self.files {
            if vf.1.file_name == file_name {
                return Some(vf.1.content.clone());
            }
        }
        None
    }

    //@todo
    pub fn get_dependencies () {

    }
    
    /*
    将当前VFS内存储的文件拷贝至temp目录下
     */
    pub fn files_copy (&self) -> Result<(), Box<dyn std::error::Error>>{

        for file in &self.files{
            let mut path: PathBuf = env::current_dir()?;
            //println!("{}", path.clone().to_str().unwrap());
            path.push(Path::new(&("temp".to_string() + "/" + &file.1.file_name)));
            //println!("{}", path.clone().to_str().unwrap());
            //path.push(Path::new(&file.1.0.file_name));
            //println!("{}", path.clone().to_str().unwrap());

            let mut file_handle = File::create(path.clone())?;
            //println!("{}", path.clone().to_str().unwrap());
            write!(file_handle, "{}", file.1.content)?;
        }

        Ok(())
    }


    /*
    编译指定路径下的文件
     */
    pub fn compile (dir: &str, file_name: &str) {

        // let flutter_project_dir = "/Users/zhen/Rust/vfs-demo/temp";
        // let output = if cfg!(target_os = "windows") {
        //     // Windows 平台 @todo
        //     Command::new("cmd")
        //         .args(&["/C", "cd", &dir, "&&", "pwd"])
        //         .output()
        //         .expect("Failed to execute command")
        // } else {
        //     // macOS 平台
        //     Command::new("sh")
        //         .args(&["-c", &format!("cd {} && rustc {}", &dir, &file_name)])
        //         .output()
        //         .expect("Failed to execute command")
        // };

        let output = Command::new("sh")
                                .args(&["-c", &format!("cd {} && rustc {}", &dir, &file_name)])
                                .output()
                                .expect("Failed to execute command");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

    }

    pub fn cargo_build() -> String {
        let mut path: PathBuf = env::current_dir().unwrap();
        path.push(Path::new(&("helloworld".to_string())));
    
        //println!("{:?}", path);
    
        // 创建一个新的 Command 实例来启动 `cargo build`
        let output = Command::new("cargo")
            .arg("build") // 添加参数 "build"
            .current_dir(path) // 设置当前工作目录为项目路径
            .output()
            .expect("Failed to execute command"); // 执行命令并等待它结束
    
        // let output = Command::new("cmd")
        //                         .args(&["/C", "cd", path.to_str().unwrap(), "&&", "cargo build"])
        //                         .output()
        //                         .expect("Failed to execute command");
    
        String::from_utf8_lossy(&output.stderr).into_owned()
    }

    /*
    运行可执行文件
     */
    pub fn run (dir: &str, file_name: &str) {

        let output = Command::new("sh")
                                .args(&["-c", &format!("cd {} && ./{}", &dir, &file_name)])
                                .output()
                                .expect("Failed to execute command");

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();
    }

}

impl History {

    pub fn new () -> History {
        History { history_stack: MyStack::<(VirtualFile, Change)>::new() }
    }

    /*
    将一项历史记录压栈
     */
    pub fn append_history (&mut self, file: VirtualFile, change: Change){
        self.history_stack.push((file, change));
    }

    /*
    查看最近一项历史记录
     */
    pub fn check_last_history (&self) -> String {
        let top = self.history_stack.top();
        match top {
            None => "No History Found".to_string(),
            Some(_) => format!("File Name: {},\nChange: {}.", top.unwrap().0.file_name, top.unwrap().1.to_string())
        }
    }

    /*
    回溯最近一项历史记录
    @todo
     */
    pub fn undo (&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let top = self.history_stack.pop();
        //let path = top.unwrap().0.path;
        //let content = top.unwrap().0.content.clone();
        //let reference = &mut top;
        match top {
            None => Ok("No History Found".to_string()),

            Some((_, Change::Modify)) => {
                let vf = top.unwrap().0;
                let mut file_handle = File::create(vf.path)?;
                //println!("{}", path.clone().to_str().unwrap());
                write!(file_handle, "{}", vf.content.clone())?;
                Ok("Modify Undo Success".to_string())
            },

            /*
            @todo
             */
            Some((_, Change::Create)) => {
                Ok("Create Undo Success".to_string())
            },

            /*
            @todo
             */
            Some((_, Change::Delete)) => {
                Ok("Delete Undo Success".to_string())
            },
        }
    }

}