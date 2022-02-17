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

async fn wrapped_main() -> Result<(), SessionError> {
    let config = Cli::new()?;

    let mut program =  Program::new(config.clone()).await?;

    if let Some(name) = config.user {
        program.set_login(name);
    }
    run(&mut program).await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    env_logger::init();

    match wrapped_main().await {
        Ok(_) => (),
        Err(e) => println!("{}", e),
    }
}
