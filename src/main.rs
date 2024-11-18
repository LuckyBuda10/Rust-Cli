use std::process;
use std::fs::{self, File};
use std::io::{stdin, stdout, Write};
use std::process::Command;
use colored::Colorize;
use open;

fn main() {
    let args = match get_user_args() {
        Ok(args) => args,
        Err(_) => {
            return main();
        }
    };

    match args.action.as_str() {
        "h" | "help" => print_available_commands(),
        "o" | "open" =>  open_file(&args),
        "w" | "write" => write_file(&args, true),
        "cls" | "clear" => clear_console(),
        "del" | "delete" => delete_file(&args.filename),
        "new" | "create" => create_file(&args.filename),
        "close" | "exit" => exit_application(),
        _ => {
            println!("{}", "Error: Invalid action.".red());
            return main();
        }
    }

    main();
}

struct CmdArgs {
    action: String,
    filename: String,
    extra_args: Vec<String>
}

impl CmdArgs {
    fn new(action: &str, filename: &str, extra_args: &Vec<String> ) -> CmdArgs {
        CmdArgs {
            action: action.to_string(),
            filename: filename.to_string(),
            extra_args: extra_args.to_vec()
        }
    }

    fn check_arg_is_valid(&self) -> bool {
        let valid_args = ["o", "open", "close", "exit", "h", "help", 
            "new", "create", "cls", "clear", "del", "delete", "w", "write", "wo", "writeover",
            "ren", "rename"
        ];
 
        if !valid_args.contains(&self.action.to_lowercase().as_str()) {
            println!("{}\nTo see all available actions, use `h` or `help`.", "Error: Invalid action.".red());
            return false;
        }

        true
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

    print!("\n{}", "C:Rust-CLI\\> ".truecolor(171, 215, 235));
    let _ = stdout().flush();

    if let Err(e) = stdin().read_line(&mut user_input) {
        println!("{}{}", "Error reading input: ".red(), e);
        return Err(());
    }

    let mut args: Vec<&str> = user_input.trim().split_whitespace().collect();

    //If the action isn't in any of these lists, then it has default arg num (2)
    let single_arg_cmds = vec!["h", "help", "close", "exit", "cls", "clear"];
    let three_arg_cmds = vec![];
    let any_num_arg_cmd = vec!["w", "write", "wo", "writeover"];

    if args.len() == 0 {
        println!("{} Arguments cannot be empty.", "Error:".red());
        return Err(());
    }

    let mut extra_arg_list: Vec<String> = Vec::new();
    let mut cmd_args = CmdArgs::new(&args[0], filename, &extra_arg_list);
    if !cmd_args.check_arg_is_valid() {
        return Err(());
    }

    if !any_num_arg_cmd.contains(&args[0]) {
        if single_arg_cmds.contains(&args[0]) {
            match has_same_arg_len(&args, &single_arg_cmds, 1, "Given action only takes 1 argument.") {
                Some(_) => {},
                None => return Err(())
            }
        } else if three_arg_cmds.contains(&args[0]) {
            match has_same_arg_len(&args, &three_arg_cmds, 3, "Given action takes 3 arguments.") {
                Some(_) => {},
                None => return Err(())
            }
        } else /*Should have 2 args*/ {
            if args.len() != 2 {
                println!("{} Given action takes 2 arguments.", "Error:".red());
                return Err(());
            }
            cmd_args.filename = args[1].to_string();
        }
    } else {
        cmd_args.filename = args[1].to_string();

        args.drain(0..=1);
        extra_arg_list = args.iter()
            .map(|arg| arg.to_string())
            .collect()
        ;

        let mut return_arg_list: Vec<String> = Vec::new();
        for arg in extra_arg_list {
            return_arg_list.push(format!("{} ", arg));
        }

        cmd_args.extra_args = return_arg_list;
    }
    
    Ok(cmd_args)
}

fn create_file(filename: &String) -> () {
    match File::create(filename) {
        Ok(_) => println!("File `{}` has been {}", filename, "created.".green()),
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
            println!("{}", "Error clearing console.".red());
            main()
        },
    }
}

fn delete_file(filename: &String) -> () {
    if let Err(e) = fs::remove_file(filename) {
        println!("{}{}", "Error deleting file: ".red(), e);
        
        return main();
    }

    println!("File `{}` has been {}", filename, "deleted.".red());
}

fn exit_application() {
    println!("{}", "Closing Application...".purple());
    process::exit(0);
}

fn print_available_commands() -> () {
    println!("{}", "-----Available Commands-----".green());
    println!("{} - Displays all avaiable arguments.", "h / help".green());
    println!("{} - Opens the given file.", "o / open".green());
    println!("{} - Clears the given text file, and writes the inputted text.", "w / write".green());
    println!("{} - Clears the current terminal.", "cls / clear".green());
    println!("{} - Deletes the given file.", "del / delete".green());
    println!("{} - Creates a new file with the given name (include file extension at the end).", "new / create".green());
    println!("{} - Exits this terminal.", "close / exit".green());
}

fn open_file(args: &CmdArgs) -> () {
    if let Err(e) = open::that(&args.filename) {
        println!("{}{}", "Error opening file: ".red(), e);

        main()
    }
}

fn write_file(args: &CmdArgs, is_writeover: bool) -> () {
    let data: String = args.extra_args.iter().map(|arg| arg.to_string()).collect();
    if is_writeover {
        match fs::write(&args.filename, data) {
            Ok(_) => println!("{} wrote to `{}`", "Successfully".green(), args.filename),
            Err(e) => println!("{}{}", "Error writing to file: ".red(), e)
        }
    } else {
        todo!()
    }
}