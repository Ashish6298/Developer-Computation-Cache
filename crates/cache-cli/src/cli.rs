use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

/// Documented stable CLI exit codes for `dcc`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
#[allow(dead_code)]
pub enum ExitCode {
    /// 0 = success / cache hit
    Success = 0,
    /// 1 = computation failed / cache entry not found
    ComputationFailed = 1,
    /// 2 = invalid configuration
    InvalidConfiguration = 2,
    /// 3 = cache storage or I/O error
    CacheError = 3,
    /// 4 = cryptographic or metadata integrity failure
    IntegrityFailure = 4,
    /// 5 = invalid arguments or unrecognized option
    InvalidArguments = 5,
}

#[allow(dead_code)]
impl ExitCode {
    pub fn as_i32(self) -> i32 {
        self as i32
    }

    pub fn exit(self) -> ! {
        std::process::exit(self as i32)
    }
}

use clap::builder::styling::{AnsiColor, Effects, Styles};

fn get_styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
        .usage(AnsiColor::Cyan.on_default().effects(Effects::BOLD))
        .literal(AnsiColor::BrightWhite.on_default().effects(Effects::BOLD))
        .placeholder(AnsiColor::BrightCyan.on_default())
        .valid(AnsiColor::Green.on_default())
        .invalid(AnsiColor::Yellow.on_default())
        .error(AnsiColor::Red.on_default().effects(Effects::BOLD))
}

const AFTER_HELP: &str = "\
\x1b[1;36mEXAMPLES:\x1b[0m
  \x1b[90m# 1. Intercept and cache any command (0 ms replay on identical hits):\x1b[0m
  $ \x1b[1;37mdcc run -i src/ -o dist/app -- cargo build --release\x1b[0m

  \x1b[90m# 2. Inspect cache hit/miss statistics and storage health diagnostics:\x1b[0m
  $ \x1b[1;37mdcc stats\x1b[0m
  $ \x1b[1;37mdcc doctor\x1b[0m

  \x1b[90m# 3. Enforce disk quota with LRU eviction and verify integrity:\x1b[0m
  $ \x1b[1;37mdcc prune --max-size 10GB --strategy lru\x1b[0m
  $ \x1b[1;37mdcc verify\x1b[0m

\x1b[1;36mDOCUMENTATION:\x1b[0m
  https://github.com/Ashish6298/Reforge
";

#[derive(Parser, Debug)]
#[command(
    name = "dcc",
    version,
    about = "Developer Computation Cache (`dcc`)\nHigh-Performance, Local-First Deterministic Computation Caching Engine",
    styles = get_styles(),
    after_help = AFTER_HELP
)]
pub struct Cli {
    #[arg(short, long, global = true, help = "Path to the cache root directory")]
    pub cache_dir: Option<PathBuf>,

    #[arg(long, global = true, help = "Output result in JSON format")]
    pub json: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Initialize local cache directory (.dcc_cache) and storage quotas")]
    Init {
        #[arg(
            short,
            long,
            help = "Max cache size (e.g. '500 MB', '2 GB', '10 GB' or raw bytes)"
        )]
        max_size: Option<String>,
    },

    #[command(about = "Execute a command with caching (0 ms replay on identical hits)")]
    Run(RunArgs),

    #[command(about = "Inspect a cached computation key, environment, and manifests")]
    Inspect {
        #[arg(help = "The SHA-256 computation key to inspect")]
        key: String,
    },

    #[command(about = "Display cache hit/miss statistics and storage metrics")]
    Stats,

    #[command(about = "Verify cryptographic SHA-256 integrity of stored CAS objects")]
    Verify,

    #[command(about = "Clean entire cache or delete specific computation keys")]
    Clean {
        #[arg(short, long, help = "Specific computation key to delete")]
        key: Option<String>,
    },

    #[command(about = "Prune unreferenced objects or enforce max cache storage budget")]
    Prune {
        #[arg(
            long,
            help = "Enforce max size (e.g. '500 MB', '2 GB', '10 GB' or raw bytes)"
        )]
        max_size: Option<String>,

        #[arg(
            long,
            default_value = "lru",
            help = "Eviction strategy: lru (least recently used), fifo (oldest created), lfu (least frequently used)"
        )]
        strategy: String,

        #[arg(
            long,
            help = "Perform a dry run without deleting any entries or objects"
        )]
        dry_run: bool,
    },

    #[command(about = "Inspect or manage cache configuration settings")]
    Config {
        #[arg(
            long,
            help = "Display configuration setting (e.g. 'max_size', 'cache_dir')"
        )]
        get: Option<String>,
    },

    #[command(about = "Diagnose cache health, environment isolation, and write permissions")]
    Doctor,

    #[command(about = "Manage and maintain local computation cache (clean, prune, verify, stats)")]
    Cache {
        #[command(subcommand)]
        command: CacheCommands,
    },
}

#[derive(Subcommand, Debug)]
pub enum CacheCommands {
    #[command(about = "Clean entire cache or delete specific keys")]
    Clean {
        #[arg(short, long, help = "Specific computation key to delete")]
        key: Option<String>,
    },

    #[command(about = "Prune unreferenced objects or enforce max cache size")]
    Prune {
        #[arg(
            long,
            help = "Enforce max size (e.g. '500 MB', '2 GB', '10 GB' or raw bytes)"
        )]
        max_size: Option<String>,

        #[arg(
            long,
            default_value = "lru",
            help = "Eviction strategy: lru (least recently used), fifo (oldest created), lfu (least frequently used)"
        )]
        strategy: String,

        #[arg(
            long,
            help = "Perform a dry run without deleting any entries or objects"
        )]
        dry_run: bool,
    },

    #[command(about = "Verify integrity of stored objects")]
    Verify,

    #[command(about = "Display cache statistics and storage metrics")]
    Stats,
}

#[derive(Args, Debug, Clone)]
pub struct RunArgs {
    #[arg(short, long = "input", action = clap::ArgAction::Append, help = "Declared input files")]
    pub inputs: Vec<String>,

    #[arg(short, long = "output", action = clap::ArgAction::Append, help = "Declared output files")]
    pub outputs: Vec<String>,

    #[arg(short, long = "env", action = clap::ArgAction::Append, help = "Declared environment variables (KEY=VAL or KEY)")]
    pub env: Vec<String>,

    #[arg(long = "op", default_value = "default", help = "Operation identifier")]
    pub operation: String,

    #[arg(long, help = "Explain cache miss reasons in detail")]
    pub explain: bool,

    #[arg(short, long, help = "Enable verbose structured debugging output")]
    pub verbose: bool,

    #[arg(
        long,
        default_value = "read-write",
        help = "Cache policy: read-write, read-only, write-only, bypass, force-recompute"
    )]
    pub policy: String,

    #[arg(
        last = true,
        required = true,
        help = "Command and arguments to execute"
    )]
    pub command: Vec<String>,
}
