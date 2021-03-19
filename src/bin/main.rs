use mcfg::actions::*;
use mcfg::error::Result;
use mcfg::shared::{user_shell, FileSystemResource, InstallerRegistry, Name, PackageRepository};
use mcfg::APP_NAME;
use std::convert::TryInto;
use std::error::Error;
use structopt::StructOpt;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, StructOpt)]
#[structopt(name = APP_NAME, about = "Machine configurator.")]
pub struct CommandLine {
    /// The level of logging to perform; from off to trace
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,

    #[structopt(subcommand)]
    sub_command: SubCommands,
}

#[derive(Debug, StructOpt)]
pub enum SubCommands {
    /// Initialize a repository to manage package-set installs
    Init {
        /// Override the local directory for the repository
        #[structopt(long, short)]
        local_dir: Option<String>,
        /// The URL to an existing repository to clone for this machine
        #[structopt(long, short)]
        repository_url: Option<String>,
    },
    /// Refresh the current repository
    Refresh,
    // --------------------------------------------------------------------------------------------
    /// Install package-sets as described in the local repository
    Install {
        /// If specified, only install package-sets from the named group
        #[structopt(long, short)]
        group: Option<Name>,
        #[structopt(long, short, requires_all = &["group"])]
        package_set: Option<Name>,
    },
    /// Update package-sets as described in the local repository
    Update {
        /// If specified, only update package-sets from the named group
        #[structopt(long, short)]
        group: Option<Name>,
        #[structopt(long, short, requires_all = &["group"])]
        package_set: Option<Name>,
    },
    /// Uninstall package-sets as described in the local repository
    Uninstall {
        /// If specified, only uninstall package-sets from the named group
        #[structopt(long, short)]
        group: Option<Name>,
        #[structopt(long, short, requires_all = &["group"])]
        package_set: Option<Name>,
    },
    /// Link any files specified in package-sets as described in the local repository
    LinkFiles {
        /// If specified, only link files in the package-sets from the named group
        #[structopt(long, short)]
        group: Option<Name>,
        #[structopt(long, short, requires_all = &["group"])]
        package_set: Option<Name>,
    },
    /// Show the current configuration
    UpdateSelf,
    // --------------------------------------------------------------------------------------------
    /// Show current path locations
    Paths,
    /// Edit the current installer registry file
    Installers,
    /// List package-sets in the local repository
    List {
        /// If specified, only list package-sets from the named group
        #[structopt(long, short)]
        group: Option<Name>,
    },
    /// Show a history of install actions on the local machine
    History {
        #[structopt(long, short)]
        limit: Option<u32>,
    },
    /// Run a shell in the repository directory, with a basic script environment
    Shell {
        #[structopt(long, short)]
        shell: Option<String>,
    },
    // --------------------------------------------------------------------------------------------
    /// Add a new package-set to the local repository
    Add {
        #[structopt(long, short)]
        as_file: bool,
        group: Name,
        package_set: Name,
    },
    /// Add an existing package-set in the local repository
    Edit { group: Name, package_set: Name },
    /// Remove an existing package-set from the local repository
    Remove { group: Name, package_set: Name },
    // --------------------------------------------------------------------------------------------
    #[cfg(feature = "remove-self")]
    CompletelyAndPermanentlyRemoveSelf,
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn parse() -> Result<Box<dyn Action>> {
    let args = CommandLine::from_args();

    pretty_env_logger::formatted_builder()
        .filter_level(match args.verbose {
            0 => log::LevelFilter::Off,
            1 => log::LevelFilter::Error,
            2 => log::LevelFilter::Warn,
            3 => log::LevelFilter::Info,
            4 => log::LevelFilter::Debug,
            _ => log::LevelFilter::Trace,
        })
        .init();

    if !args.sub_command.is_init() && !is_initialized() {
        eprintln!(
            "Error: your local repository is not initialized, try running the 'init' command"
        );
        panic!("Could not continue");
    }

    args.sub_command.try_into()
}

pub fn is_initialized() -> bool {
    InstallerRegistry::default_path().is_file() && PackageRepository::default_path().is_dir()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl TryInto<Box<dyn Action>> for SubCommands {
    type Error = mcfg::error::Error;

    fn try_into(self) -> std::result::Result<Box<dyn Action>, Self::Error> {
        match self {
            // ----------------------------------------------------------------------------------------
            // Repository Commands
            // ----------------------------------------------------------------------------------------
            SubCommands::Init {
                local_dir,
                repository_url,
            } => InitAction::new_action(local_dir, repository_url),
            SubCommands::Refresh => RefreshAction::new_action(),
            SubCommands::Add {
                group,
                package_set,
                as_file,
            } => ManageAction::add_action(group, package_set, as_file),
            SubCommands::Edit { group, package_set } => {
                ManageAction::edit_action(group, package_set)
            }
            SubCommands::Remove { group, package_set } => {
                ManageAction::remove_action(group, package_set)
            }
            SubCommands::List { group } => ListAction::new_action(group),
            // ----------------------------------------------------------------------------------------
            // Package Commands
            // ----------------------------------------------------------------------------------------
            SubCommands::Install { group, package_set } => {
                InstallAction::install_action(group, package_set)
            }
            SubCommands::Update { group, package_set } => {
                InstallAction::update_action(group, package_set)
            }
            SubCommands::Uninstall { group, package_set } => {
                InstallAction::uninstall_action(group, package_set)
            }
            SubCommands::LinkFiles { group, package_set } => {
                InstallAction::link_files_action(group, package_set)
            }
            // ----------------------------------------------------------------------------------------
            // Installer Commands
            // ----------------------------------------------------------------------------------------
            SubCommands::Installers => EditInstallersAction::new_action(),
            SubCommands::History { limit } => HistoryAction::new_action(limit),
            SubCommands::UpdateSelf => UpdateSelfAction::new_action(),
            // ----------------------------------------------------------------------------------------
            // Help Commands
            // ----------------------------------------------------------------------------------------
            SubCommands::Paths => ShowPathsAction::new_action(),
            #[cfg(feature = "remove-self")]
            SubCommands::CompletelyAndPermanentlyRemoveSelf => RemoveSelfAction::new_action(),
            SubCommands::Shell { shell } => {
                ShellAction::new_action(&shell.unwrap_or_else(user_shell))
            }
        }
    }
}

impl SubCommands {
    fn is_init(&self) -> bool {
        matches!(self, SubCommands::Init { .. })
    }
}

// ------------------------------------------------------------------------------------------------
// Start Here!
// ------------------------------------------------------------------------------------------------

fn main() -> std::result::Result<(), Box<dyn Error>> {
    mcfg::reporter::set_is_interactive(true);
    parse()?.run()?;
    Ok(())
}
