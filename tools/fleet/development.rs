use std::path::{Path, PathBuf};
use std::process::Command;

use crate::{AppResult, OutputMode, context};

pub(crate) fn repository(args: &[String]) -> AppResult<PathBuf> {
    let cwd = crate::optional_value(args, "--repo").map_or_else(
        || std::env::current_dir().map_err(|error| error.to_string()),
        |path| Ok(PathBuf::from(path)),
    )?;
    git(&cwd, &["rev-parse", "--show-toplevel"]).map(PathBuf::from)
}

fn git(repo: &Path, args: &[&str]) -> AppResult<String> {
    let output = Command::new("git")
        .arg("--no-optional-locks")
        .args(["-C", &repo.to_string_lossy()])
        .args(args)
        .output()
        .map_err(|error| format!("run git: {error}"))?;
    if !output.status.success() {
        return Err(format!("Git observation unavailable: {}", args.join(" ")));
    }
    String::from_utf8(output.stdout)
        .map(|text| text.trim().to_owned())
        .map_err(|error| format!("Git output is not UTF-8: {error}"))
}

pub(crate) fn remote_identity(remote: &str) -> String {
    if let Some((scheme, address)) = remote.split_once("://") {
        let address = address.rsplit_once('@').map_or(address, |(_, host)| host);
        // URLs with query/fragment credentials are not useful repository identities.
        let address = address.split(['?', '#']).next().unwrap_or(address);
        format!("{scheme}://{address}")
    } else {
        remote.to_owned()
    }
}

pub(crate) fn current(repo: &Path, mode: OutputMode, verify: bool) -> AppResult<()> {
    let branch = git(repo, &["branch", "--show-current"])?;
    let status = git(
        repo,
        &["status", "--porcelain=v1", "--untracked-files=normal"],
    )?;
    let worktrees = git(repo, &["worktree", "list", "--porcelain"])?;
    let verification = context::verify_repo(repo);
    let facts = [
        ("repository", git(repo, &["rev-parse", "--show-toplevel"])?),
        (
            "branch",
            if branch.is_empty() {
                "DETACHED".to_owned()
            } else {
                branch
            },
        ),
        ("head", git(repo, &["rev-parse", "HEAD"])?),
        ("change_entries", status.lines().count().to_string()),
        (
            "worktrees",
            worktrees
                .lines()
                .filter(|line| line.starts_with("worktree "))
                .count()
                .to_string(),
        ),
        (
            "origin",
            git(repo, &["remote", "get-url", "origin"]).map_or_else(
                |_| "UNAVAILABLE".to_owned(),
                |remote| remote_identity(&remote),
            ),
        ),
        (
            "cached_origin_main",
            git(repo, &["rev-parse", "--verify", "refs/remotes/origin/main"])
                .unwrap_or_else(|_| "UNAVAILABLE".to_owned()),
        ),
        (
            "cached_main_divergence",
            git(
                repo,
                &["rev-list", "--left-right", "--count", "origin/main...HEAD"],
            )
            .map(|value| value.replace('\t', " "))
            .unwrap_or_else(|_| "UNAVAILABLE".to_owned()),
        ),
        (
            "context_sha",
            verification
                .context_sha
                .clone()
                .unwrap_or_else(|| "UNAVAILABLE".to_owned()),
        ),
        (
            "context",
            if verification.passed() {
                "PASS"
            } else {
                "FAIL"
            }
            .to_owned(),
        ),
        (
            "remote_observation",
            "LOCAL_CACHE_ONLY; fetch separately for freshness".to_owned(),
        ),
    ];
    for (key, value) in facts {
        match mode {
            OutputMode::Human => println!("{key}: {value}"),
            OutputMode::Machine => println!("{key}\t{value}"),
        }
    }
    if verify && !verification.passed() {
        return Err(verification.errors.join("\n"));
    }
    Ok(())
}

pub(crate) fn verify(repo: &Path) -> AppResult<()> {
    for args in [
        vec!["fmt", "--all", "--check"],
        vec![
            "clippy",
            "--workspace",
            "--all-targets",
            "--locked",
            "--",
            "-D",
            "warnings",
        ],
        vec!["test", "--workspace", "--locked"],
    ] {
        println!("Checking cargo {}", args.join(" "));
        let status = Command::new("cargo")
            .args(&args)
            .current_dir(repo)
            .status()
            .map_err(|error| format!("run cargo: {error}"))?;
        if !status.success() {
            return Err(format!("cargo {} failed", args.join(" ")));
        }
    }
    context::command_verify(repo, OutputMode::Human)?;
    let status = Command::new("sh")
        .arg(repo.join("scripts/proof.sh"))
        .current_dir(repo)
        .env("RUST_REPS_VERIFY_FROM_XTASK", "1")
        .status()
        .map_err(|error| format!("run product proof: {error}"))?;
    if !status.success() {
        return Err("product transcript/install proof failed".to_owned());
    }
    for args in [vec!["diff", "--check"], vec!["diff", "--cached", "--check"]] {
        let status = Command::new("git")
            .args(&args)
            .current_dir(repo)
            .status()
            .map_err(|error| format!("run diff check: {error}"))?;
        if !status.success() {
            return Err(format!("git {} failed", args.join(" ")));
        }
    }
    println!(
        "Rust Reps verification passed. Learning quality and real-terminal feel require human review."
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remote_observation_redacts_url_credentials() {
        assert_eq!(
            remote_identity(
                "https://person:secret@github.com/example/project.git?token=secret#secret"
            ),
            "https://github.com/example/project.git"
        );
        assert_eq!(
            remote_identity("git@github.com:example/project.git"),
            "git@github.com:example/project.git"
        );
    }
}
