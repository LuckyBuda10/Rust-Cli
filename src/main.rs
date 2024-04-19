use std::{fs, process};
use std::fs::File;
use std::io::{stdin, stdout, Write};
use std::process::Command;
use colored::Colorize;
use open;

/* Todo List:
    - Append to file
    - Write to file (overwrites text)
*/

fn main() {
    let args = match get_user_args() {
        Ok(args) => args,
        Err(_) => {
            return main();
        }
    };

    match args.action.as_str() {
        "o" | "open" =>  open_file(&args),
        "h" | "help" => print_available_commands(),
        "c" | "create" => create_file(&args.filename),
        "cls" | "clear" => clear_console(),
        "e" | "exit" => exit_application(),
        "d" | "delete" => delete_file(&args.filename),
        _ => {
            println!("Invalid command!");
            return main();
        }
    }

    main();
}

struct CmdArgs {
    action: String,
    filename: String
}

impl CmdArgs {
    fn new(action: &str, filename: &str ) -> CmdArgs {
        CmdArgs {
            action: action.to_string(),
            filename: filename.to_string(),
        }
    }

    fn check_arg_is_valid(&self) -> bool {
        let valid_args = ["o", "open", "e", "exit", "h", "help", 
            "c", "create", "cls", "clear", "d", "delete"
        ];
 
        if !valid_args.contains(&self.action.to_lowercase().as_str()) {
            println!("{}\nTo see all available actions, use `h` or `help`.", "Invalid action.".red());
            return false;
        }

        true
    }
}

fn create_file(filename: &String) -> () {
    match File::create(filename) {
        Ok(_) => println!("File `{}` has been created.", filename),
        Err(e) => {
            println!("{}", String::from(format!("{}{}", "Error creating file ".red(), e).as_str()));
            main()
        }
    }
}


fn clear_console() -> () {
    let clear_status = if cfg!(windows) {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg("clear")
            .status()
    };

    match clear_status {
        Ok(exit_status) if exit_status.success() => (),
        _ => { 
            println!("Error clearing console screen.");
            main()
        },
    }
}

fn delete_file(filename: &String) -> () {
    if let Err(e) = fs::remove_file(filename) {
        println!("{}{}", "Error deleting file: ".red(), e);
        
        return main();
    }

    println!("File `{}` has been deleted.", filename);
}

fn exit_application() {
    println!("{}", "Closing Application...".truecolor(171, 215, 235));
    process::exit(0);
}

fn print_available_commands() -> () {
    println!("{}", "-----Available Commands-----".truecolor(144, 181, 168));
    println!("{} - Creates a new file with the given name (include file type at the end).", "c / create".truecolor(144, 181, 168));
    println!("{} - Clears the current terminal.", "cls / clear".truecolor(144, 181, 168));
    println!("{} - Exits this terminal.", "e / exit".truecolor(144, 181, 168));
    println!("{} - Displays all avaiable arguments.", "h / help".truecolor(144, 181, 168));
    println!("{} - Opens the given file.", "o / open".truecolor(144, 181, 168));
}

fn open_file(args: &CmdArgs) -> () {
    if let Err(e) = open::that(&args.filename) {
        println!("{}{}", "Error opening file: ".red(), e);

        main()
    }
}

fn has_same_arg_len(args: &Vec<&str>, arr_to_check: &Vec<&str>, len: usize, err_msg: &str) -> Option<()> {
    if arr_to_check.contains(&args[0].to_lowercase().as_str())  {
        if args.len() != len {
            println!("{}", String::from(format!("{}{}", "Error: ".red(), err_msg).as_str()));
            return None;
        }

        return Some(());
    }

    None
}

fn get_user_args() -> Result<CmdArgs, ()> {
    let filename: &str = "none";
    let mut user_input: String = String::new();

    print!("{}", "C:\\Users\\LuckyBuda10\\Desktop\\Command_Line> ".truecolor(171, 215, 235));
    let _ = stdout().flush();

    if let Err(e) = stdin().read_line(&mut user_input) {
        println!("{}{}", "Error reading input: ".red(), e);
        return Err(());
    }

    let args: Vec<&str> = user_input.trim().split_whitespace().collect();

    let single_arg_cmds = vec!["h", "help", "e", "exit", "cls", "clear"];
    let three_arg_cmds = vec![];

    if args.len() == 0 {
        println!("{}", "Error: Arguments cannot be empty.".red());
        return Err(());
    }

    let mut cmd_args = CmdArgs::new(&args[0], filename);
    if !cmd_args.check_arg_is_valid() {
        return Err(());
    }

    if single_arg_cmds.contains(&args[0]) {
        match has_same_arg_len(&args, &single_arg_cmds, 1, "Error: Given action only takes 1 argument.") {
            Some(_) => {},
            None => return Err(())
        }
    } else if three_arg_cmds.contains(&args[0]) {
        match has_same_arg_len(&args, &three_arg_cmds, 3, "Error: Given action takes 3 arguments.") {
            Some(_) => {},
            None => return Err(())
        }
    } else {
        if args.len() != 2 {
            println!("{} Given action takes 2 arguments.", "Error:".red());
            return Err(());
        }
        cmd_args.filename = args[1].to_string();
    }

    Ok(cmd_args)
}