use crate::catalog;

pub enum DrillCommand {
    Practice,
    Run(&'static catalog::Drill),
    List,
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
    catalog::print_drill_list();
}

pub fn print_help() {
    println!("Rust Reps");
    println!();
    println!("Usage:");
    println!("  rust-reps");
    println!("  rust-reps [DRILL_ID]");
    println!("  rust-reps play");
    println!("  rust-reps --list");
    println!("  rust-reps --help");
    println!("  rust-reps --version");
    println!();
    println!("Default:");
    println!("  Starts a practice session.");
    println!();
    println!("Current drills:");
    for drill in catalog::drills() {
        println!("{}  {}", drill.id, drill.expected);
    }
}

pub fn print_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}
