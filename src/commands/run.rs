use std::process::Command;

pub fn run(args: Vec<String>) {
    println!("{:#?}", args);
    let cmd = Command::new("sh").arg("-c").arg(args.join(" ")).output().unwrap();
    println!("{:#?}", cmd)
}
