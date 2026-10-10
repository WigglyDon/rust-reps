use crate::{catalog, engine, orchestration, presentation};

#[expect(
    clippy::large_enum_variant,
    reason = "parsed once per launch; the owned exercise packet needs no extra allocation"
)]
pub enum DrillCommand {
    Practice,
    Run(engine::ExercisePacket),
    List,
    Tui,
    Lab,
    Workshop,
    Help,
    Version,
}

pub fn select_command(args: &[&str]) -> Result<DrillCommand, String> {
    if args.len() > 1 {
        return Err("expected at most one argument".to_string());
    }

    let id = args.first().copied().unwrap_or("play");
    if id == "play" || id == "plain" {
        return Ok(DrillCommand::Practice);
    }
    if id == "--list" {
        return Ok(DrillCommand::List);
    }
    if id == "tui" {
        return Ok(DrillCommand::Tui);
    }
    if id == "lab" {
        return Ok(DrillCommand::Lab);
    }
    if id == "workshop" {
        return Ok(DrillCommand::Workshop);
    }
    if id == "--help" || id == "-h" {
        return Ok(DrillCommand::Help);
    }
    if id == "--version" {
        return Ok(DrillCommand::Version);
    }
    if id.starts_with('-') {
        return Err(format!("unknown argument: {}", id));
    }

    orchestration::current_product_exercise_packet_by_id(id)
        .map(DrillCommand::Run)
        .map_err(|error| error.to_string())
}

pub fn print_drill_list() {
    presentation::print_drill_list();
}

pub fn print_help() {
    presentation::print_help_text();
    println!("Current drills:");
    for drill in catalog::drills() {
        if drill.stimulus.is_empty() {
            println!("{}", drill.id);
        } else {
            println!("{}  {}", drill.id, drill.stimulus);
        }
    }
}

pub fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}
