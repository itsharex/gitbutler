use std::path::PathBuf;

#[derive(Debug, clap::Parser)]
#[clap(name = "gitbutler-cli", about = "A CLI for GitButler", version = option_env!("GIX_VERSION"))]
pub struct Args {
    /// Enable tracing for debug and performance information printed to stderr.
    #[clap(short = 'd', long)]
    pub trace: bool,
    /// Run as if gitbutler-cli was started in PATH instead of the current working directory.
    #[clap(short = 'C', long, default_value = ".", value_name = "PATH")]
    pub current_dir: PathBuf,

    #[clap(subcommand)]
    pub cmd: Subcommands,
}

#[derive(Debug, clap::Subcommand)]
pub enum Subcommands {
    /// Update the local workspace against an updated remote or target branch.
    Status {
        /// Also compute unified diffs for each tree-change.
        #[clap(long, short = 'd')]
        unified_diff: bool,
    },
    /// Calculate the changes between
    CommitChanges {
        /// Also compute unified diffs for each tree-change.
        #[clap(long, short = 'd')]
        unified_diff: bool,
        /// The revspec to the commit that the returned changes turn the previous commit into.
        current_commit: String,
        /// The revspec to the previous commit that the returned changes transform into current commit.
        previous_commit: Option<String>,
    },
    /// Return the dependencies of worktree changes with the commits that last changed them.
    #[clap(visible_alias = "dep")]
    HunkDependency,
    /// Returns the list of stacks that are currently part of the GitButler workspace.
    Stacks,
    StackBranches {
        id: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clap() {
        use clap::CommandFactory;
        Args::command().debug_assert();
    }
}
