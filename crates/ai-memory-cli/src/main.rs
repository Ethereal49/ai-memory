//! `ai-memory` binary entry point.
//!
//! Deliberately thin: all logic lives in the `ai_memory_cli` lib target so it
//! is unit-testable and linkable. See that crate's docs for the dispatch flow.

#![doc(html_no_source)]

use anyhow::Result;

fn main() -> Result<()> {
    #[cfg(windows)]
    {
        // Windows executables start with a much smaller main-thread stack than
        // Unix processes. The CLI dispatch future contains every subcommand
        // branch and can outgrow that default as the command tree expands.
        // Keep the platform default for worker threads, but give the one
        // thread that owns the top-level future enough room to dispatch it.
        std::thread::Builder::new()
            .name("ai-memory-main".to_owned())
            .stack_size(8 * 1024 * 1024)
            .spawn(run)?
            .join()
            .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
    }

    #[cfg(not(windows))]
    run()
}

fn run() -> Result<()> {
    ai_memory_cli::run_blocking()
}
