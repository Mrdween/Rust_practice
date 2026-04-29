use std::env;

fn get_env_or_default(key: &str) -> Result<String, env::VarError> {
    env::var(key)
}
fn main () {
    let greeting = String::from("hello");
    println!("There is something or err {:?}", get_env_or_default(&greeting));
}
