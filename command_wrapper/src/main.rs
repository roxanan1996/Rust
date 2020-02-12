use Wrapper::ProcessWrapper;
mod Wrapper;


fn main() {
    println!("Hello, world!");

    let p = ProcessWrapper {command: "airodump-ng".to_string(),
    interface: "wlp2s0".to_string()};

    let output = p.create_command().output().expect("Command error");
    println!("{:?}", output);
}
