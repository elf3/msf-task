use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// 一个任务动作的枚举
pub enum Action {
    /// 写入任务到文件中
    Add {
        /// struct 宏生成代码
        #[structopt()]
        text: String,
    },
    /// 按位置移除任务
    Done {
        #[structopt()]
        position: usize,
    },
    /// 显示所有任务
    List,
}


#[derive(Debug, StructOpt)]
#[structopt(
name = "run task",
about = "A command line Run Task"
)]
pub struct CommandLineArgs {
    #[structopt(subcommand)]
    pub action: Action,

    #[structopt(parse(from_os_str), short, long)]
    pub file: Option<PathBuf>,
}

