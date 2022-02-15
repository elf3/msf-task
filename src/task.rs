use std::fmt;
use std::fmt::{Formatter};
use std::fs::{File, OpenOptions};
use std::io::{Error, ErrorKind, Result, Seek, SeekFrom};
use std::path::PathBuf;
use serde::Deserialize;
use serde::Serialize;
use chrono::{
    DateTime,
    Local,
    serde::ts_seconds,
    Utc,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub text: String,
    #[serde(with = "ts_seconds")]
    pub created: DateTime<Utc>,
}
fn collect_tasks(mut file: &File) -> Result<Vec<Task>> {
    file.seek(SeekFrom::Start(0))?; // Rewind the file before.
    let tasks = match serde_json::from_reader(file) {
        Ok(tasks) => tasks,
        Err(e) if e.is_eof() => Vec::new(),
        Err(e) => Err(e)?,
    };
    file.seek(SeekFrom::Start(0))?; // Rewind the file after.
    Ok(tasks)
}
pub fn add_task(path: PathBuf, task: Task) -> Result<()> {
    let  file = OpenOptions::new().read(true).write(true).create(true).open(path)?;

    let mut tasks: Vec<Task> = collect_tasks(&file)?;

    tasks.push(task);
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}

pub fn finish_task(path: PathBuf, postion: usize) -> Result<()> {
    let  file = OpenOptions::new().read(true).write(true).open(path)?;

    let mut tasks: Vec<Task> = collect_tasks(&file)?;
    if postion == 0 || postion > tasks.len() {
        return Err(Error::new(ErrorKind::InvalidInput, "invalid task id"));
    }

    tasks.remove(postion - 1);
    file.set_len(0)?;
    serde_json::to_writer(file, &tasks)?;
    Ok(())
}


pub fn list_task(path: PathBuf) -> Result<()> {
    let file = OpenOptions::new().read(true).open(path)?;
    let tasks = collect_tasks(&file)?;
    if tasks.is_empty() {
        println!("task is empty")
    } else {
        let mut order: u32 = 1;
        for task in tasks {
            println!("{} : {}", order, task);
            order += 1;
        }
    }
    Ok(())
}


impl Task {
    pub fn new(text: String) -> Task {
        let created = Utc::now();
        Task {
            text,
            created,
        }
    }
}




impl fmt::Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let created_at = self.created.with_timezone(&Local).format("%F %H:%M");
        write!(f, "{:<50} [{}]",self.text, created_at)
    }
}