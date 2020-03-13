use clap::Clap;
use std::fs;
use std::path::Path;

#[derive(Clap, Debug)]
#[clap(name = "rigit")]
/// The stupid content tracker
enum Subcommand {
    /// Add file contents to the index
    Add,
    /// Apply a series of patches from a mailbox
    Am,
    /// Create an archive of files from a named tree
    Archive,
    /// Use binary search to find the commit that introduced a bug
    Bisect,
    /// List, create, or delete branches
    Branch,
    /// Move objects and refs by archive
    Bundle,
    /// Switch branches or restore working tree files
    Checkout,
    /// Apply the changes introduced by some existing commits
    #[clap(name = "cherry-pick")]
    Cherrypick,
    /// Graphical alternative to git-commit
    Citool,
    /// Remove untracked files from the working tree
    Clean,
    /// Clone a repository into a new directory
    Clone,
    /// Record changes to the repository
    Commit,
    /// Give an object a human readable name based on an available ref
    Describe,
    /// Show changes between commits, commit and working tree, etc
    Diff,
    /// Download objects and refs from another repository
    Fetch,
    /// Prepare patches for e-mail submission
    #[clap(name = "format-patch")]
    Formatpatch,
    /// Cleanup unnecessary files and optimize the local repository
    Gc,
    /// The Git repository browser
    Gitk,
    /// Print lines matching a pattern
    Grep,
    /// A portable graphical interface to Git
    Gui,
    /// Create an empty Git repository or reinitialize an existing one
    Init,
    /// Show commit logs
    Log,
    /// Join two or more development histories together
    Merge,
    /// Move or rename a file, a directory, or a symlink
    Mv,
    /// Add or inspect object notes
    Notes,
    /// Fetch from and integrate with another repository or a local branch
    Pull,
    /// Update remote refs along with associated objects
    Push,
    /// Compare two commit ranges (e.g. two versions of a branch)
    #[clap(name = "range-diff")]
    Rangediff,
    /// Reapply commits on top of another base tip
    Rebase,
    /// Reset current HEAD to the specified state
    Reset,
    /// Revert some existing commits
    Revert,
    /// Remove files from the working tree and from the index
    Rm,
    /// Summarize 'git log' output
    Shortlog,
    /// Show various types of objects
    Show,
    /// Stash the changes in a dirty working directory away
    Stash,
    /// Show the working tree status
    Status,
    /// Initialize, update or inspect submodules
    Submodule,
    /// Create, list, delete or verify a tag object signed with GPG
    Tag,
    /// Manage multiple working trees
    Worktree,
    /// Compute object ID and optionally creates a blob from a file
    #[clap(name = "hash-object")]
    Hashobject,
}

fn main() {
    let matches = Subcommand::parse();
    dbg!(&matches);
    match matches {
        Subcommand::Add => (),
        Subcommand::Am => (),
        Subcommand::Archive => (),
        Subcommand::Bisect => (),
        Subcommand::Branch => (),
        Subcommand::Bundle => (),
        Subcommand::Checkout => (),
        Subcommand::Cherrypick => (),
        Subcommand::Citool => (),
        Subcommand::Clean => (),
        Subcommand::Clone => (),
        Subcommand::Commit => (),
        Subcommand::Describe => (),
        Subcommand::Diff => (),
        Subcommand::Fetch => (),
        Subcommand::Formatpatch => (),
        Subcommand::Gc => (),
        Subcommand::Gitk => (),
        Subcommand::Grep => (),
        Subcommand::Gui => (),
        Subcommand::Init => init(),
        Subcommand::Log => (),
        Subcommand::Merge => (),
        Subcommand::Mv => (),
        Subcommand::Notes => (),
        Subcommand::Pull => (),
        Subcommand::Push => (),
        Subcommand::Rangediff => (),
        Subcommand::Rebase => (),
        Subcommand::Reset => (),
        Subcommand::Revert => (),
        Subcommand::Rm => (),
        Subcommand::Shortlog => (),
        Subcommand::Show => (),
        Subcommand::Stash => (),
        Subcommand::Status => (),
        Subcommand::Submodule => (),
        Subcommand::Tag => (),
        Subcommand::Worktree => (),
        Subcommand::Hashobject => (),
    }
}

// TODO: Real git allows us to safely "re-initialise" a repository. We as of now don't.
// TODO make this code cleaner
fn init() {
    if Path::new(".rigit").exists() {
        println!("Existing rigit project exists");
    } else {
        let directories_to_create = [
            ".rigit/refs/heads",
            ".rigit/refs/tags",
            ".rigit/objects/info",
            ".rigit/objects/pack",
            ".rigit/hooks",
            ".rigit/branches",
            ".rigit/info",
        ]
        .iter();
        let files_to_create = [
            ".rigit/config",
            ".rigit/description",
            ".rigit/HEAD",
            ".rigit/info/exclude",
        ]
        .iter();
        for directory in directories_to_create {
            fs::create_dir_all(directory).unwrap();
        }
        for file in files_to_create {
            fs::File::create(file).unwrap();
        }
        println!("Initialised rigit repository");
    }
}
