use std::env;
use std::process;
use cli_tool::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err|{
        eprintln!("Process passing args {}", err);
        //Process helps exit our code without panicing 
        process::exit(1);
    });
   

/*  
    This is also another way to propergate the error gotten from the run function.
    if let Err(e) = run(config)  {
        println!("Application error {}", e);
        process::exit(1);
    } 
*/
    let hhh = cli_tool::run(config);
    match hhh {
        Err(e) => {   eprintln!("Application error {}", e);
                                     process::exit(1);
                                    }
        Ok(()) => (),
    };
        


}



