use crate::{catalog, presentation};

pub enum DrillCommand {
    Practice,
    Run(&'static catalog::Drill),
    List,
    Tui,
    Help,
    Version,
}

pub fn select_command(args: &[&str]) -> Result<DrillCommand, String> {
    if args.len() > 1 {
        return Err("expected at most one argument".to_string());
    }

    let id = args.first().copied().unwrap_or("play");
    if id == "play" {
        return Ok(DrillCommand::Practice);
    }
    if id == "--list" {
        return Ok(DrillCommand::List);
    }
    if id == "tui" {
        return Ok(DrillCommand::Tui);
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

    catalog::find_drill(id)
        .map(DrillCommand::Run)
        .ok_or_else(|| format!("unknown drill id: {}", id))
}

pub fn print_drill_list() {
    presentation::print_drill_list();
}

pub fn print_help() {
    presentation::print_help_text();
    println!("Current drills:");
    for drill in catalog::drills() {
        println!("{}  {}", drill.id, drill.expected);
    }
}

pub fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}
