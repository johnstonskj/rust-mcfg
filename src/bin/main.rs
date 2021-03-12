use mcfg::actions::*;
use mcfg::error::Result;
use mcfg::shared::{InstallerRegistry, PackageRepository};
use mcfg::APP_NAME;
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
        group: Option<String>,
        #[structopt(long, short, requires_all = &["group"])]
        package_set: Option<String>,
    },
    /// Update package-sets as described in the local repository
    Update {
        /// If specified, only update package-sets from the named group
        #[structopt(long, short)]
        group: Option<String>,
        #[structopt(long, short, requires_all = &["group"])]
        package_set: Option<String>,
    },
    /// Uninstall package-sets as described in the local repository
    Uninstall {
        /// If specified, only uninstall package-sets from the named group
        #[structopt(long, short)]
        group: Option<String>,
        #[structopt(long, short, requires_all = &["group"])]
        package_set: Option<String>,
    },
    /// Link any files specified in package-sets as described in the local repository
    LinkFiles {
        /// If specified, only link files in the package-sets from the named group
        #[structopt(long, short)]
        group: Option<String>,
        #[structopt(long, short, requires_all = &["group"])]
        package_set: Option<String>,
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
        group: Option<String>,
    },
    /// Show a history of install actions on the local machine
    History {
        #[structopt(long, short)]
        limit: Option<u32>,
    },
    // --------------------------------------------------------------------------------------------
    /// Add a new package-set to the local repository
    Add {
        #[structopt(long, short)]
        as_file: bool,
        group: String,
        package_set: String,
    },
    /// Add an existing package-set in the local repository
    Edit { group: String, package_set: String },
    /// Remove an existing package-set from the local repository
    Remove { group: String, package_set: String },
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

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

    match args.sub_command {
        // ----------------------------------------------------------------------------------------
        // Repository Commands
        // ----------------------------------------------------------------------------------------
        SubCommands::Init {
            local_dir,
            repository_url,
        } => InitAction::new(local_dir, repository_url),
        SubCommands::Refresh => RefreshAction::new(),
        SubCommands::Add {
            group,
            package_set,
            as_file,
        } => ManageAction::add(group, package_set, as_file),
        SubCommands::Edit { group, package_set } => ManageAction::edit(group, package_set),
        SubCommands::Remove { group, package_set } => ManageAction::remove(group, package_set),
        SubCommands::List { group } => ListAction::new(group),
        // ----------------------------------------------------------------------------------------
        // Package Commands
        // ----------------------------------------------------------------------------------------
        SubCommands::Install { group, package_set } => InstallAction::install(group, package_set),
        SubCommands::Update { group, package_set } => InstallAction::update(group, package_set),
        SubCommands::Uninstall { group, package_set } => {
            InstallAction::uninstall(group, package_set)
        }
        SubCommands::LinkFiles { group, package_set } => {
            InstallAction::link_files(group, package_set)
        }
        // ----------------------------------------------------------------------------------------
        // Installer Commands
        // ----------------------------------------------------------------------------------------
        SubCommands::Installers => EditInstallersAction::new(),
        SubCommands::History { limit } => HistoryAction::new(limit),
        SubCommands::UpdateSelf => UpdateSelfAction::new(),
        // ----------------------------------------------------------------------------------------
        // Help Commands
        // ----------------------------------------------------------------------------------------
        SubCommands::Paths => ShowPathsAction::new(),
    }
}

pub fn is_initialized() -> bool {
    InstallerRegistry::default_path().is_file() && PackageRepository::default_path().is_dir()
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl SubCommands {
    fn is_init(&self) -> bool {
        matches!(self, SubCommands::Init { .. })
    }
}

// ------------------------------------------------------------------------------------------------
// Start Here!
// ------------------------------------------------------------------------------------------------

fn main() -> std::result::Result<(), Box<dyn Error>> {
    parse()?.run()?;
    Ok(())
}
