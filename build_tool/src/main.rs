//! FeatherCore Build Tool - Main Entry
//! FeatherCore 构建工具主入口
//! 
//! This tool is used to:
//! 1. Parse board configuration files from platform directory
//! 2. Generate linker scripts (link.x) for boot and kernel
//! 3. Parse device tree files
//! 4. Build boot or kernel images
//!
//! # Usage
//! ```bash
//! # List supported boards
//! feathercore-build -r /path/to/FeatherCore list-boards
//!
//! # Generate configuration for a board
//! feathercore-build -r /path/to/FeatherCore generate stm32f429i-disc
//!
//! # Build boot image
//! feathercore-build -r /path/to/FeatherCore build stm32f429i-disc boot
//!
//! # Build kernel image
//! feathercore-build -r /path/to/FeatherCore build stm32f429i-disc kernel
//!
//! # Build all (boot + kernel)
//! feathercore-build -r /path/to/FeatherCore build stm32f429i-disc all
//!
//! # Clean build artifacts
//! feathercore-build -r /path/to/FeatherCore clean
//! ```

use std::env;

mod root_path;
mod config;
mod device_tree;
mod linker;
mod build;
mod utils;

const VERSION: &str = "0.1.0";

/// Print help information
/// 打印帮助信息
fn print_help() {
    println!("FeatherCore Build Tool v{}", VERSION);
    println!();
    println!("Usage: feathercore-build [OPTIONS] [COMMAND] [ARGS]");
    println!();
    println!("Options:");
    println!("  -r, --root <PATH>    FeatherCore root directory path (required)");
    println!("  -h, --help           Print this help");
    println!("  -v, --version        Print version");
    println!();
    println!("Commands:");
    println!("  list-boards                    List supported boards");
    println!("  show-board <NAME>              Show board information");
    println!("  generate [BOARD]               Generate configuration for board");
    println!("  build [BOARD] [all|boot|kernel] Build for board");
    println!("  clean                          Clean build");
    println!();
    println!("Examples:");
    println!("  feathercore-build -r /home/user/FeatherCore/FeatherCore list-boards");
    println!("  feathercore-build -r /home/user/FeatherCore/FeatherCore generate stm32f429i-disc");
    println!("  feathercore-build -r /home/user/FeatherCore/FeatherCore build stm32f429i-disc all");
    println!("  feathercore-build -r /home/user/FeatherCore/FeatherCore clean");
}

/// Print version information
/// 打印版本信息
fn print_version() {
    println!("FeatherCore Build Tool v{}", VERSION);
}

/// Main entry point
/// 主入口函数
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    // Parse root path parameter (-r, --root)
    // 解析根路径参数 (-r, --root)
    let mut root_path: Option<String> = None;
    let mut cmd_args: Vec<String> = Vec::new();
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-r" | "--root" => {
                if i + 1 < args.len() {
                    root_path = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    println!("Error: -r/--root requires a path argument");
                    return;
                }
            }
            _ => {
                cmd_args.push(args[i].clone());
                i += 1;
            }
        }
    }
    
    // Set root path
    // 设置根路径
    if let Some(path) = root_path {
        root_path::set_root_path(&path);
    }
    
    if cmd_args.is_empty() {
        print_help();
        return;
    }
    
    match cmd_args[0].as_str() {
        "-h" | "--help" => print_help(),
        "-v" | "--version" => print_version(),
        "list-boards" => build::list_boards(),
        "show-board" => {
            if cmd_args.len() > 1 {
                build::show_board_info(&cmd_args[1]);
            } else {
                println!("Error: Board name required");
                print_help();
            }
        }
        "generate" => {
            let board_name = if cmd_args.len() > 1 { &cmd_args[1] } else { "stm32f429i-disc" };
            build::generate_config(board_name);
        }
        "build" => {
            let board_name = if cmd_args.len() > 1 { &cmd_args[1] } else { "stm32f429i-disc" };
            let target = if cmd_args.len() > 2 { cmd_args[2].as_str() } else { "all" };
            build::build(board_name, target);
        }
        "clean" => build::clean_build(),
        _ => {
            println!("Error: Unknown command '{}'", cmd_args[0]);
            print_help();
        }
    }
}
