use std::process::{Command, Stdio};
use semver::Version;

fn extract_from_string(string:String) -> Option<Version>{
    for part in string.trim().split_whitespace(){
        if let Ok(version) = Version::parse(
            part.trim_prefix("v.")
                .trim_prefix("V.")
                .trim_prefix('v')
                .trim_prefix("V")){
            return Some(version)
        }
    }
    None
}

fn extract_from_command(program:&str,argument:&str) -> Option<Version>{
    if let Ok(result) = Command::new(program)
        .env_clear()
        .stdin(Stdio::null())
        .arg(argument)
        .output() {
        if let Ok(output) = String::from_utf8(result.stdout) {
            match extract_from_string(output) {
                Some(version) => Some(version),
                None => if let Ok(output) = String::from_utf8(result.stderr) {
                    extract_from_string(output)
                } else {
                    None
                }
            }
        }
        else{
            None
        }
    }
    else{
        None
    }
}

pub fn extract_version(program_file:String) -> Option<Version>{
    if let Some(version) = extract_from_command(&program_file,"--version")
    {
        Some(version)
    }
    else if let Some(version) = extract_from_command(&program_file,"-V"){
        Some(version)
    }
    else {
        None
    }
}
