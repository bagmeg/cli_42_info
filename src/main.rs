mod cli;
mod program;

use cli::Cli;
use cli_42::SessionError;
use program::Command;
use program::Program;

// TODO:
// Event 만들어주는 library 찾아보기 예) enum_derive? strum?
async fn run(prog: &mut Program) -> Result<(), SessionError> {
    let command = prog.config.command.to_owned();
    let cmd = command.trim().to_uppercase();
    match cmd.as_str() {
        "ME" => prog.run_program(Command::Me).await?,
        "ID" => prog.run_program(Command::Id).await?,
        "EMAIL" => prog.run_program(Command::Email).await?,
        "EVENT" => prog.run_program(Command::Event).await?,
        "LOGIN" => prog.run_program(Command::Login).await?,
        "POINT" => prog.run_program(Command::CorrectionPoint).await?,
        "LEVEL" => prog.run_program(Command::Level).await?,
        "WALLET" => prog.run_program(Command::Wallet).await?,
        "LOCATION" => prog.run_program(Command::Location).await?,
        "BLACKHOLE" => prog.run_program(Command::Blackhole).await?,
        "COMMAND" => prog.config.list_available_commands(),
        _ => println!("Command `{}` not found", command),
    }
    Ok(())
}

// TODO:
// - add wrapper_main() -> Result<(), SessionError>
#[tokio::main]
async fn main() {
    env_logger::init();

    let config = match Cli::new() {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let mut program = match Program::new(config.clone()).await {
        Ok(program) => program,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    if let Some(name) = config.user {
        program.set_login(name);
    }
    match run(&mut program).await {
        Ok(_) => (),
        Err(err) => println!("{}", err),
    }
}
