use std::fmt;
use std::process::exit;

#[derive(Debug)]
pub enum Errcode {
    ContainerError(u8),
    NotSupported(u8),
    ArgumentInvalid(&'static str),
}

#[allow(unreachable_patterns)]
// trait Display, allows Errcode enum to be displayed by:
//      println!("{}", error);
//  in this case, it calls the function "fmt", which we define the behaviour below
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Define what behaviour for each variant of the enum
        match &self {
            // Message to display when an argument is invalid
            Errcode::ArgumentInvalid(element) => write!(f, "ArgumentInvalid: {}", element),

            _ => write!(f, "{:?}", self), // For any variant not previously covered
        }
    }
}

impl Errcode {
    pub fn get_retcode(&self) -> i32 {
        match &self {
            _ => 1,
        }
    }
}

// Get the result from a function, and exit the process with the correct error code
pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        // If it's a success, return 0
        Ok(_) => {
            log::debug!("Exit without any error, returning 0");
            exit(0);
        }

        // If there's an error, print an error message and return the retcode
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit:\n\t{}\n\tReturning {}", e, retcode);
            exit(retcode);
        }
    }
}
