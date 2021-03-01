use std::fs::File;
use std::io::Read;
use std::path::Path;

fn main() {
    let mut f = File::open(Path::new(env!("TASKDATA")).parent().unwrap().join("export.json")).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let tasks = task_hookrs::import::import(s.as_bytes()).unwrap();
    assert!(task_hookrs::tw::save(&tasks).is_ok());
}
