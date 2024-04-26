use std::process::{ Command, Stdio };

pub fn run(args: Vec<String>) {
    println!("{:#?}", args);
    let cmd = Command::new("sh")
        .args(["-c", &args.join(" ")])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();
    println!("ID: {:?}", cmd.id());
    let result = cmd.wait_with_output().unwrap();
    match result {
        output => {
            let output: Vec<u8> = output.stdout;

            let mut new_output: String = String::new();

            for key in output {
                let key: u32 = u32::from(key);
                let key: char = char::from_u32(key).unwrap();
                new_output += &key.to_string().as_str();
            }

            println!("{:?}", new_output.trim());
        }
    }
}
