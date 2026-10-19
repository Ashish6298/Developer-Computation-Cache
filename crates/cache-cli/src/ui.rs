//! Terminal UI and Welcome Hub for `dcc` CLI
//! Provides a high-contrast terminal experience inspired by developer workflow hubs.

#[cfg(windows)]
pub fn enable_ansi_support() {
    unsafe {
        use std::ffi::c_void;
        type Handle = *mut c_void;
        type Dword = u32;
        extern "system" {
            fn GetStdHandle(n_std_handle: Dword) -> Handle;
            fn GetConsoleMode(h_console_handle: Handle, lp_mode: *mut Dword) -> i32;
            fn SetConsoleMode(h_console_handle: Handle, dw_mode: Dword) -> i32;
        }
        const STD_OUTPUT_HANDLE: Dword = 0xFFFFFFF5;
        const ENABLE_VIRTUAL_TERMINAL_PROCESSING: Dword = 0x0004;

        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut mode: Dword = 0;
        if GetConsoleMode(handle, &mut mode) != 0 {
            let _ = SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

// ---------------------------------------------------------------------------
// Palette: Gunmetal
// ---------------------------------------------------------------------------
const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const GRAY: &str = "\x1b[90m";
const WHITE: &str = "\x1b[37m";

const FACE: &str = "\x1b[38;2;140;155;170m"; // logo blocks
const EDGE: &str = "\x1b[38;2;70;80;92m"; // logo outline / shadow
const ACCENT: &str = "\x1b[38;2;150;165;180m"; // commands, headings, highlights

/// Renders the "DCC" logo: solid blocks in FACE, box-drawing outline in EDGE.
fn render_logo() -> String {
    const D: [&str; 6] = [
        "██████╗ ",
        "██╔══██╗",
        "██║  ██║",
        "██║  ██║",
        "██████╔╝",
        "╚═════╝ ",
    ];
    const C: [&str; 6] = [
        " ██████╗",
        "██╔════╝",
        "██║     ",
        "██║     ",
        "╚██████╗",
        " ╚═════╝",
    ];

    let mut out = String::new();
    for i in 0..6 {
        let row = format!("{}  {}  {}", D[i], C[i], C[i]);
        out.push_str("  ");
        for ch in row.chars() {
            match ch {
                ' ' => out.push(' '),
                '█' => {
                    out.push_str(FACE);
                    out.push(ch);
                }
                _ => {
                    out.push_str(EDGE);
                    out.push(ch);
                }
            }
        }
        out.push_str(RESET);
        out.push('\n');
    }
    out
}

/// Prints the terminal welcome hub when `dcc` is invoked without subcommands.
pub fn print_welcome_hub() {
    #[cfg(windows)]
    enable_ansi_support();

    let logo = render_logo();

    let banner = format!(
r#"
{logo}
  {BOLD}Developer Computation Cache (`dcc`){RESET}
  {GRAY}High-Performance, Local-First Deterministic Computation Caching Engine{RESET}

  {WHITE}A universal, content-addressed process cache that eliminates redundant CPU cycles.{RESET}
  {WHITE}Fingerprints computations and re-executes identical runs instantly in {ACCENT}0 ms{RESET}{WHITE}.{RESET}

  {GRAY}─────────────────────────────────────────────────────────────────────────────{RESET}

  {BOLD}1. Intercept & Cache Any Command:{RESET}

     {ACCENT}$ dcc run -i <inputs...> -o <outputs...> -- <command...>{RESET}

     • {BOLD}When to use:{RESET} Wrapping slow compilers, code generators, linters, or tests.
     • {BOLD}Why to use:{RESET}  Caches outputs in CAS; identical runs replay in {ACCENT}0 ms{RESET}.


  {BOLD}2. Inspect Cache Health & Metrics:{RESET}

     {ACCENT}$ dcc stats{RESET}   {GRAY}·{RESET}   {ACCENT}$ dcc doctor{RESET}

     • {BOLD}When to use:{RESET} Viewing hit/miss ratios, storage footprint, and system health.
     • {BOLD}Why to use:{RESET}  Real-time visibility into disk savings and engine diagnostics.

  {GRAY}─────────────────────────────────────────────────────────────────────────────{RESET}
  {GRAY}v1.0.0  ·  rust 1.77+  ·  mit / apache-2.0  ·  github.com/Ashish6298/Reforge{RESET}
  {GRAY}─────────────────────────────────────────────────────────────────────────────{RESET}

  {GRAY}╭─────────────────────────────────────────────────────────────────────────╮{RESET}
  {GRAY}│{RESET}  {BOLD}Explore all features:{RESET} run {ACCENT}dcc --help{RESET}                                   {GRAY}│{RESET}
  {GRAY}│  (init, clean, prune, verify, inspect, config, doctor, and more)        │{RESET}
  {GRAY}╰─────────────────────────────────────────────────────────────────────────╯{RESET}
"#
    );

    println!("{}", banner);
}

/// Prints the categorized CLI Help Reference when `--help` or `-h` is requested.
pub fn print_help() {
    #[cfg(windows)]
    enable_ansi_support();

    let help_text = format!(
r#"
  {BOLD}Developer Computation Cache (`dcc`){RESET}
  {GRAY}High-Performance, Local-First Deterministic Computation Caching Engine{RESET}

  {BOLD}{ACCENT}USAGE:{RESET}
    {WHITE}dcc [OPTIONS] <COMMAND>{RESET}

  {BOLD}{ACCENT}CORE WORKFLOW{RESET}
    {BOLD}run{RESET}       {WHITE}Execute a command with caching{RESET} {ACCENT}(0 ms replay on hit){RESET}
    {BOLD}init{RESET}      {WHITE}Initialize local cache directory (.dcc_cache) and storage quota{RESET}

  {BOLD}{ACCENT}INSPECTION & HEALTH{RESET}
    {BOLD}stats{RESET}     {WHITE}Display hit/miss ratios, storage footprint & CPU savings{RESET}
    {BOLD}doctor{RESET}    {WHITE}Diagnose permissions, sandboxing & engine integrity{RESET}
    {BOLD}inspect{RESET}   {WHITE}Deep-dive into cryptographic keys & manifest diffs{RESET}

  {BOLD}{ACCENT}STORAGE & MAINTENANCE{RESET}
    {BOLD}prune{RESET}     {WHITE}Enforce disk quota using LRU, LFU, or FIFO eviction{RESET}
    {BOLD}verify{RESET}    {WHITE}Cryptographic SHA-256 audit across all stored objects{RESET}
    {BOLD}clean{RESET}     {WHITE}Purge cached objects or delete specific computation keys{RESET}
    {BOLD}config{RESET}    {WHITE}Inspect or modify runtime storage configuration{RESET}

  {BOLD}{ACCENT}GLOBAL OPTIONS{RESET}
    {ACCENT}-c, --cache-dir <DIR>{RESET}    {GRAY}Override cache storage directory (or DCC_CACHE_DIR){RESET}
    {ACCENT}    --json{RESET}               {GRAY}Output machine-readable JSON results{RESET}
    {ACCENT}-h, --help{RESET}               {GRAY}Print this help reference{RESET}
    {ACCENT}-V, --version{RESET}            {GRAY}Print version information{RESET}

  {GRAY}─────────────────────────────────────────────────────────────────────────────{RESET}

  {BOLD}{ACCENT}QUICK EXAMPLES{RESET}
    {GRAY}# Wrap & cache any deterministic build command:{RESET}
    {ACCENT}$ dcc run -i src/ -o dist/app -- cargo build --release{RESET}

    {GRAY}# Inspect system health, cache efficiency, and disk footprint:{RESET}
    {ACCENT}$ dcc stats   ·   $ dcc doctor{RESET}

    {GRAY}# Reclaim disk space and verify CAS object integrity:{RESET}
    {ACCENT}$ dcc prune --max-size 10GB --strategy lru   ·   $ dcc verify{RESET}

  {GRAY}─────────────────────────────────────────────────────────────────────────────{RESET}
  {GRAY}Docs: github.com/Ashish6298/Reforge  ·  Run 'dcc <command> --help' for subcommands{RESET}
"#
    );

    println!("{}", help_text);
}
