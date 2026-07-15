use briefcase::cli::{schedule::ScheduleCommands, Cli, Commands};
use clap::Parser;

#[test]
fn test_schedule_args_start() {
    let cli = Cli::parse_from(["briefcase", "schedule", "start"]);

    match cli.command {
        Commands::Schedule(args) => {
            assert!(matches!(args.command, ScheduleCommands::Start(_)));
        }
        _ => panic!("expected schedule command"),
    }
}
