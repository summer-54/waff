
pub const UNIX_SOCKET_PATH: &str = "/tmp/waff_daemon";
pub const INSTANCE_FOLDER: &str = ".waff";
pub const API_URL: &str = "https://inf54.ru/api";

pub fn tasks_path(instance_path: &String) -> String {
    format!("{instance_path}/tasks")
}
pub fn task_path(tasks_path: &String, litera: &String) -> String {
    format!("{tasks_path}/{litera}")
}
pub fn contest_path(instance_path: &String) -> String {
    format!("{instance_path}/contest.json")
}
pub fn info_path(task_path: &String) -> String {
    format!("{task_path}/info.json")
}
pub fn samples_path(task_path: &String) -> String {
    format!("{task_path}/samples")
}
pub fn statements_path(task_path: &String) -> String {
    format!("{task_path}/statements")
}
pub fn statement_path(statements_path: &String, name: &String) -> String {
    format!("{statements_path}/{name}")
}
pub fn input_path(tests_path: &String, test_number: u32) -> String {
    format!("{tests_path}/{test_number}.in")
}
pub fn output_path(tests_path: &String, test_number: u32) -> String {
    format!("{tests_path}/{test_number}.out")
}

pub fn default_tasks_path() -> String {
    tasks_path(&INSTANCE_FOLDER.to_string())
}
pub fn default_task_path(litera: &String) -> String {
    task_path(&default_tasks_path(), &litera)
}
pub fn default_contest_path() -> String {
    contest_path(&INSTANCE_FOLDER.to_string())
}
pub fn default_info_path(litera: &String) -> String {
    info_path(&default_task_path(litera))
}
pub fn default_samples_path(litera: &String) -> String {
    samples_path(&default_task_path(litera))
}
pub fn default_statements_path(litera: &String) -> String {
    statements_path(&default_task_path(litera))
}
pub fn default_statement_path(litera: &String, name: &String) -> String {
    statement_path(&default_task_path(litera), name)
}
pub fn defaults_input_path(litera: &String, test_number: u32) -> String {
    input_path(&default_samples_path(litera), test_number)
}
pub fn defaults_output_path(litera: &String, test_number: u32) -> String {
    output_path(&default_samples_path(litera), test_number)
}
