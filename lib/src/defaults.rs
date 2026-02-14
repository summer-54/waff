pub const UNIX_SOCKET_PATH: &str = "/tmp/waff_daemon";
pub const INSTANCE_FOLDER: &str = ".waff";
pub const API_URL: &str = "https://contest.inf54.ru/api";

pub fn tasks_path(instance_path: &str) -> Box<str> {
    format!("{instance_path}/tasks").into()
}
pub fn task_path(tasks_path: &str, litera: &str) -> Box<str> {
    format!("{tasks_path}/{litera}").into()
}
pub fn contest_path(instance_path: &str) -> Box<str> {
    format!("{instance_path}/contest.json").into()
}
pub fn info_path(task_path: &str) -> Box<str> {
    format!("{task_path}/info.json").into()
}
pub fn samples_path(task_path: &str) -> Box<str> {
    format!("{task_path}/samples").into()
}
pub fn statements_path(task_path: &str) -> Box<str> {
    format!("{task_path}/statements").into()
}
pub fn statement_path(statements_path: &str, name: &str) -> Box<str> {
    format!("{statements_path}/{name}").into()
}
pub fn input_path(tests_path: &str, test_number: u32) -> Box<str> {
    format!("{tests_path}/{test_number}.in").into()
}
pub fn output_path(tests_path: &str, test_number: u32) -> Box<str> {
    format!("{tests_path}/{test_number}.out").into()
}

pub fn default_tasks_path() -> Box<str> {
    tasks_path(INSTANCE_FOLDER)
}
pub fn default_task_path(litera: &str) -> Box<str> {
    task_path(&default_tasks_path(), litera)
}
pub fn default_contest_path() -> Box<str> {
    contest_path(INSTANCE_FOLDER)
}
pub fn default_info_path(litera: &str) -> Box<str> {
    info_path(&default_task_path(litera))
}
pub fn default_samples_path(litera: &str) -> Box<str> {
    samples_path(&default_task_path(litera))
}
pub fn default_statements_path(litera: &str) -> Box<str> {
    statements_path(&default_task_path(litera))
}
pub fn default_statement_path(litera: &str, name: &str) -> Box<str> {
    statement_path(&default_task_path(litera), name)
}
pub fn defaults_input_path(litera: &str, test_number: u32) -> Box<str> {
    input_path(&default_samples_path(litera), test_number)
}
pub fn defaults_output_path(litera: &str, test_number: u32) -> Box<str> {
    output_path(&default_samples_path(litera), test_number)
}
