use std::process::{exit, Command};
use names::Generator;


fn update_commit_push(){

    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git add command.");

    if !add_command.status.success(){
        eprintln!("Error: Failed to add files to the repo.");
        exit(1)
    }

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("Failed to execute git commit command");

    if !commit_command.status.success(){
        eprintln!("Error: Failed to commit changes");
        exit(1)
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Failed to execute git push command");

    if !push_command.status.success(){
        eprintln!("Error: Failed to push files to the repo.");
        exit(1)
    }

    println!("Successfully added, commited and pushed changes.")
}

fn name_generator() -> String{
    let mut generator = Generator::default;

    generator().next().unwrap()
}

fn main() {
    update_commit_push();
}
