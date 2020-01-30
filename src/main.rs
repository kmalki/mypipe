extern crate clap;
use clap::Arg;
use clap::App;
use std::process::Command;
use std::process::Stdio;

fn main() {
    let matches = App::new("mypipe").version("1.0").author("Kamel Malki")
	          .arg(Arg::with_name("input")
			   .long("in")
			   .takes_value(true)
			   .required(true))
	           .arg(Arg::with_name("output")
			   .long("out")
			   .takes_value(true)
			   .required(true))
	           .get_matches();


    let arg1 = matches.value_of("input")
	        .unwrap();
    
    let arg2 = matches.value_of("output")
                 .unwrap();

    let arg1_result = Command::new(arg1)
                       .stdout(Stdio::piped())
                       .spawn()
                       .expect("shell no execute");

    let arg2_result = Command::new(arg2)
                 .stdin(arg1_result.stdout.unwrap())
                 .output()
		 .expect("shell no execute");
    
    println!("{}",String::from_utf8_lossy(&(arg2_result).stdout));
}
