use std::process::Command;


// class for Wrappers
pub struct ProcessWrapper {
    command: String,
    interface: String,
    // TODO
    // list of arguments in a list to use command.args...
}

impl ProcessWrapper {

    pub fn new(command: String, interface: String) -> ProcessWrapper {
        ProcessWrapper {command: command.clone(), interface: interface.clone()}
    }

    pub fn create_command(&self) -> &mut Command {
        Command::new(self.command.clone())
            .arg(self.interface.clone())
    }

//    pub fn get_output(comm: Command) -> Result<Output, Error> {
  //      comm.output()
    //}
}

