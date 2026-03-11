//! FeatherCore 构建工具
//! FeatherCore Build Tool
//! 
//! 主机可执行程序，负责：
//! Host executable program responsible for:
//! 1. 根据板级配置文件生成链接脚本和编译配置
//!    Generating linker scripts and compilation configurations based on board configuration files
//! 2. 启动 boot 或 kernel 镜像的构建
//!    Initiating the build process for bootloader or kernel images

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::collections::HashMap;

/// Device tree node
/// 设备树节点
#[derive(Debug, Clone)]
struct DeviceTreeNode {
    /// Node name
    /// 节点名称
    name: String,
    /// Node path
    /// 节点路径
    path: String,
    /// Properties
    /// 属性
    properties: Vec<Property>,
    /// Children nodes
    /// 子节点
    children: Vec<DeviceTreeNode>,
}

/// Device tree property
/// 设备树属性
#[derive(Debug, Clone)]
struct Property {
    /// Property name
    /// 属性名称
    name: String,
    /// Property value
    /// 属性值
    value: PropertyValue,
}

/// Property value types
/// 属性值类型
#[derive(Debug, Clone)]
enum PropertyValue {
    /// String value
    /// 字符串值
    String(String),
    /// Integer value
    /// 整数值
    Integer(u64),
    /// Boolean value
    /// 布尔值
    Boolean(bool),
    /// Array of integers
    /// 整数数组
    IntegerArray(Vec<u64>),
}

const VERSION: &str = "0.1.0";

/// 主函数
/// Main function
/// 
/// 解析命令行参数并执行相应的命令
/// Parses command line arguments and executes the corresponding command
fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    match args[1].as_str() {
        "-h" | "--help" => print_help(),
        "-v" | "--version" => print_version(),
        "list-boards" => list_boards(),
        "show-board" => {
            if args.len() > 2 {
                show_board_info(&args[2]);
            } else {
                println!("Error: Board name required");
                print_help();
            }
        }
        "generate" => {
            let board_name = if args.len() > 2 { &args[2] } else { "stm32f429i-disc" };
            generate_config(board_name);
        }
        "build" => {
            let board_name = if args.len() > 2 { &args[2] } else { "stm32f429i-disc" };
            let target = if args.len() > 3 { args[3].as_str() } else { "all" };
            build(board_name, target);
        }
        "clean" => clean_build(),
        _ => {
            println!("Error: Unknown command '{}'", args[1]);
            print_help();
        }
    }
}

/// 打印帮助信息
/// Print help information
/// 
/// 显示构建工具的使用方法和可用命令
/// Displays the usage and available commands for the build tool
fn print_help() {
    println!("FeatherCore Build Tool v{}", VERSION);
    println!();
    println!("Usage: feathercore-build [COMMAND] [OPTIONS]");
    println!();
    println!("Commands:");
    println!("  list-boards                    List supported boards");
    println!("  show-board <NAME>              Show board information");
    println!("  generate [BOARD]               Generate configuration for board");
    println!("  build [BOARD] [all|boot|kernel] Build for board");
    println!("  clean                          Clean build");
    println!("  -h, --help                     Print this help");
    println!("  -v, --version                  Print version");
    println!();
    println!("Examples:");
    println!("  feathercore-build list-boards");
    println!("  feathercore-build show-board stm32f429i-disc");
    println!("  feathercore-build generate stm32f429i-disc");
    println!("  feathercore-build build stm32f429i-disc all");
    println!("  feathercore-build clean");
}

/// 打印版本信息
/// Print version information
/// 
/// 显示构建工具的版本号
/// Displays the version number of the build tool
fn print_version() {
    println!("FeatherCore Build Tool v{}", VERSION);
}

/// 列出支持的开发板
/// List supported boards
/// 
/// 扫描board目录，列出所有支持的开发板
/// Scans the board directory and lists all supported boards
fn list_boards() {
    let board_dir = PathBuf::from("board");
    
    println!("Supported boards:");
    
    // 查找所有板级配置文件
    // Find all board configuration files
    let mut boards = Vec::new();
    
    if let Ok(entries) = fs::read_dir(board_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let vendor_dir = entry.path();
                let boards_dir = vendor_dir.join("boards/vendor");
                
                if boards_dir.exists() {
                    if let Ok(vendor_entries) = fs::read_dir(boards_dir) {
                        for vendor_entry in vendor_entries.flatten() {
                            if vendor_entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                                let board_name = vendor_entry.file_name().to_string_lossy().to_string();
                                boards.push(board_name);
                            }
                        }
                    }
                }
            }
        }
    }
    
    // 去重并排序
    // Deduplicate and sort
    boards.sort();
    boards.dedup();
    
    for board in &boards {
        println!("  - {}", board);
    }
    
    if boards.is_empty() {
        println!("  No boards found");
    }
}

/// 显示开发板信息
/// Show board information
/// 
/// 显示指定开发板的配置信息
/// Displays configuration information for the specified board
fn show_board_info(board_name: &str) {
    let config_path = find_board_config(board_name);
    
    match config_path {
        Some(path) => {
            println!("Board: {}", board_name);
            println!("Config file: {:?}", path);
            
            // 读取配置文件
            // Read configuration file
            if let Ok(content) = fs::read_to_string(&path) {
                println!("Configuration:");
                for line in content.lines() {
                    let line = line.trim();
                    if !line.is_empty() && !line.starts_with('#') {
                        println!("  {}", line);
                    }
                }
            } else {
                println!("Error: Could not read config file");
            }
        }
        None => {
            println!("Error: Board '{}' not found", board_name);
            println!("Available boards:");
            list_boards();
        }
    }
}

/// 查找开发板配置文件
/// Find board configuration file
/// 
/// 搜索指定开发板的配置文件
/// Searches for the configuration file for the specified board
fn find_board_config(board_name: &str) -> Option<PathBuf> {
    let board_dir = PathBuf::from("board");
    
    // 搜索所有可能的路径
    // Search all possible paths
    let patterns = vec![
        format!("{}/config/board.toml", board_name),
        format!("{}/board.toml", board_name),
        format!("vendor/{}/config/board.toml", board_name),
        format!("vendor/{}/board.toml", board_name),
        // 支持嵌套目录结构
        // Support nested directory structure
        format!("*/boards/vendor/{}/config/board.toml", board_name),
        format!("*/boards/vendor/{}/board.toml", board_name),
    ];
    
    for pattern in patterns {
        let path = board_dir.join(pattern);
        if path.exists() {
            return Some(path);
        }
    }
    
    // 递归搜索所有子目录
    // Recursively search all subdirectories
    if let Ok(entries) = fs::read_dir(board_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let vendor_dir = entry.path().join("boards/vendor").join(board_name).join("config/board.toml");
                if vendor_dir.exists() {
                    return Some(vendor_dir);
                }
            }
        }
    }
    
    None
}

/// 生成配置
/// Generate configuration
/// 
/// 根据开发板配置生成链接脚本和设备树信息
/// Generates linker scripts and device tree information based on board configuration
fn generate_config(board_name: &str) {
    println!("Generating configuration for {}...", board_name);
    
    let config_path = find_board_config(board_name);
    
    match config_path {
        Some(path) => {
            // 确保目录存在
            // Ensure directories exist
            fs::create_dir_all("boot").unwrap_or_default();
            fs::create_dir_all("kernel").unwrap_or_default();
            fs::create_dir_all("common/src/generated").unwrap_or_default();
            
            // 读取配置文件
            // Read configuration file
            if let Ok(content) = fs::read_to_string(&path) {
                // 解析TOML格式配置
                // Parse TOML configuration
                let config = parse_toml_config(&content);
                
                // 获取配置值，使用默认值
                // Get configuration values, use defaults if not present
                // 支持新旧变量名
                // Support both old and new variable names
                let flash_base = config.get("bootloader.boot_base_address")
                    .or_else(|| config.get("bootloader.code_base"))
                    .or_else(|| config.get("bootloader_base"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x08000000);
                
                let boot_size = config.get("bootloader.boot_size")
                    .or_else(|| config.get("bootloader.code_size"))
                    .or_else(|| config.get("bootloader_size"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x10000);
                
                let kernel_base = config.get("kernel.kernel_base_address")
                    .or_else(|| config.get("kernel.code_base"))
                    .or_else(|| config.get("kernel_base"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x08010000);
                
                let kernel_size = config.get("kernel.kernel_size")
                    .or_else(|| config.get("kernel.code_size"))
                    .or_else(|| config.get("kernel_size"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x1F0000);
                
                let stack_size = config.get("kernel.kernel_stack_size")
                    .or_else(|| config.get("kernel.stack_size"))
                    .or_else(|| config.get("kernel_stack_size"))
                    .or_else(|| config.get("stack_size"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x4000);
                
                let heap_size = config.get("kernel.kernel_heap_size")
                    .or_else(|| config.get("kernel.heap_size"))
                    .or_else(|| config.get("kernel_heap_size"))
                    .or_else(|| config.get("heap_size"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x10000);
                
                let sram_base = config.get("memory.sram_base")
                    .or_else(|| config.get("sram_base"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x20000000);
                
                let sram_size = config.get("memory.sram_size")
                    .or_else(|| config.get("sram_size"))
                    .map(|s| parse_hex_or_decimal(s))
                    .unwrap_or(0x40000);
                
                // 确定架构类型
                // Determine architecture type
                let arch_type = get_arch_type(board_name);
                
                // 生成boot链接脚本
                // Generate boot linker script
                let boot_script = generate_linker_script(
                    &arch_type,
                    flash_base,
                    boot_size,
                    sram_base,
                    sram_size,
                    4 * 1024, // 4K stack size for bootloader
                    0 // No heap for bootloader
                );
                
                // 生成kernel链接脚本
                // Generate kernel linker script
                let kernel_script = generate_linker_script(
                    &arch_type,
                    kernel_base,
                    kernel_size,
                    sram_base,
                    sram_size,
                    stack_size,
                    heap_size
                );
                
                // 写入文件
                // Write files
                fs::write("boot/link.x", boot_script).unwrap_or_default();
                fs::write("kernel/link.x", kernel_script).unwrap_or_default();
                
                // 处理设备树
                // Process device tree
                if let Some(dts_path) = find_device_tree_file(board_name) {
                    if let Ok(dts_content) = fs::read_to_string(&dts_path) {
                        if let Ok(device_tree_info) = generate_device_tree_info(&dts_content) {
                            fs::write("common/src/generated/devicetree.rs", device_tree_info).unwrap_or_default();
                            println!("Generated device tree information:");
                            println!("  - common/src/generated/devicetree.rs");
                        } else {
                            println!("Warning: Failed to generate device tree information");
                        }
                    } else {
                        println!("Warning: Could not read device tree file");
                    }
                } else {
                    println!("Warning: Device tree file not found");
                }
                
                println!("Generated linker scripts:");
                println!("  - boot/link.x");
                println!("  - kernel/link.x");
                println!("Configuration generated successfully!");
            } else {
                println!("Error: Could not read config file");
            }
        }
        None => {
            println!("Error: Board '{}' not found", board_name);
        }
    }
}

/// 查找设备树文件
/// Find device tree file
/// 
/// 搜索指定开发板的设备树文件
/// Searches for the device tree file for the specified board
fn find_device_tree_file(board_name: &str) -> Option<PathBuf> {
    let board_dir = PathBuf::from("board");
    
    // 搜索所有可能的路径
    // Search all possible paths
    let patterns = vec![
        format!("{}/config/devicetree.dts", board_name),
        format!("{}/devicetree.dts", board_name),
        format!("vendor/{}/config/devicetree.dts", board_name),
        format!("vendor/{}/devicetree.dts", board_name),
        // 支持嵌套目录结构
        // Support nested directory structure
        format!("*/boards/vendor/{}/config/devicetree.dts", board_name),
        format!("*/boards/vendor/{}/devicetree.dts", board_name),
    ];
    
    for pattern in patterns {
        let path = board_dir.join(pattern);
        if path.exists() {
            return Some(path);
        }
    }
    
    // 递归搜索所有子目录
    // Recursively search all subdirectories
    if let Ok(entries) = fs::read_dir(board_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let vendor_dir = entry.path().join("boards/vendor").join(board_name).join("config/devicetree.dts");
                if vendor_dir.exists() {
                    return Some(vendor_dir);
                }
            }
        }
    }
    
    None
}

/// 生成设备树信息
/// Generate device tree information
/// 
/// 解析DTS文件并生成Rust代码形式的设备树信息
/// Parses DTS file and generates device tree information in Rust code form
fn generate_device_tree_info(dts_content: &str) -> Result<String, String> {
    // 简单的DTS解析实现
    // Simple DTS parsing implementation
    let mut root = DeviceTreeNode {
        name: "root".to_string(),
        path: "/".to_string(),
        properties: Vec::new(),
        children: Vec::new(),
    };
    
    let mut current_path = "/".to_string();
    let mut stack: Vec<*mut DeviceTreeNode> = Vec::new();
    let mut current_node: *mut DeviceTreeNode = &mut root;
    
    for line in dts_content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") || line.starts_with("/*") || line.starts_with("*") {
            continue;
        }
        
        if line.starts_with("/") && line.ends_with("{") {
            // 根节点
            let name = line.strip_suffix(" {").unwrap_or(&line).trim_start_matches("/");
            if !name.is_empty() {
                unsafe {
                    (*current_node).name = name.to_string();
                }
            }
        } else if line.starts_with("}") {
            // 结束节点
            if let Some(parent) = stack.pop() {
                current_node = parent;
                current_path = current_path.rsplit_once('/').unwrap_or(("", "/")).0.to_string();
                if current_path.is_empty() {
                    current_path = "/".to_string();
                }
            }
        } else if line.ends_with("{") {
            // 开始新节点
            let name = line.strip_suffix(" {").unwrap_or(&line);
            // 提取节点名称，去除冒号和后面的内容
            let node_name = name.split('@').next().unwrap_or(name).split(':').next().unwrap_or(name).trim().to_string();
            let node_path = if current_path == "/" {
                format!("/{}", node_name)
            } else {
                format!("{}/{}", current_path, node_name)
            };
            
            let new_node = DeviceTreeNode {
                name: node_name,
                path: node_path.clone(),
                properties: Vec::new(),
                children: Vec::new(),
            };
            
            unsafe {
                stack.push(current_node);
                (*current_node).children.push(new_node);
                current_node = (*current_node).children.last_mut().unwrap();
            }
            current_path = node_path;
        } else if line.contains("=") {
            // 属性
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() != 2 {
                continue;
            }
            
            let name = parts[0].trim().to_string();
            let value_str = parts[1].trim();
            
            // 处理原始值
            let mut processed_value = value_str.to_string();
            
            // 先去除注释，再去除分号，最后检查尖括号
            // 去除注释
            if let Some(comment_pos) = processed_value.find("/*") {
                processed_value = processed_value[..comment_pos].trim().to_string();
            }
            
            // 去除所有分号
            processed_value = processed_value.replace(';', "").trim().to_string();
            
            let value = if processed_value.starts_with('<') && processed_value.ends_with('>') {
                // 提取尖括号内的内容
                let content = processed_value.trim_matches(&['<', '>'][..]);
                // 去除注释
                let content = content.split("/*").next().unwrap_or(content).trim();
                let parts: Vec<&str> = content.split_whitespace().collect();
                
                // 检查是否是布尔值
                if parts.len() == 1 {
                    if parts[0] == "true" {
                        PropertyValue::Boolean(true)
                    } else if parts[0] == "false" {
                        PropertyValue::Boolean(false)
                    } else if let Ok(num) = parse_hex_or_decimal_u64(parts[0]) {
                        // 单个整数（支持十六进制）
                        PropertyValue::Integer(num)
                    } else {
                        // 如果解析失败，作为字符串处理
                        PropertyValue::String(processed_value)
                    }
                } else {
                    // 尝试解析为整数数组（支持十六进制）
                    let mut array = Vec::new();
                    let parts_len = parts.len();
                    for part in &parts {
                        if let Ok(num) = parse_hex_or_decimal_u64(part) {
                            array.push(num);
                        } else {
                            // 如果有任何部分无法解析为整数，就作为字符串处理
                            break;
                        }
                    }
                    if !array.is_empty() && array.len() == parts_len {
                        PropertyValue::IntegerArray(array)
                    } else {
                        PropertyValue::String(processed_value)
                    }
                }
            } else if processed_value == "true" {
                // 布尔值 true
                PropertyValue::Boolean(true)
            } else if processed_value == "false" {
                // 布尔值 false
                PropertyValue::Boolean(false)
            } else if processed_value.starts_with('"') && processed_value.ends_with('"') {
                // 字符串
                let s = processed_value.trim_matches('"').to_string();
                PropertyValue::String(s)
            } else {
                // 其他类型，作为字符串处理
                PropertyValue::String(processed_value)
            };
            
            unsafe {
                (*current_node).properties.push(Property {
                    name,
                    value,
                });
            }
        }
    }
    
    // 生成Rust代码
    // Generate Rust code
    let mut rust_code = String::new();
    rust_code.push_str("//! Generated device tree information\n");
    rust_code.push_str("//! 生成的设备树信息\n");
    rust_code.push_str("\n");
    rust_code.push_str("use crate::devicetree::{DeviceTreeNode, Property, PropertyValue};\n");
    rust_code.push_str("\n");
    rust_code.push_str("/// Pre-parsed device tree\n");
    rust_code.push_str("/// 预解析的设备树\n");
    rust_code.push_str("pub static DEVICE_TREE: DeviceTreeNode = DeviceTreeNode {\n");
    rust_code.push_str(&format!("    name: \"{}\",\n", root.name));
    rust_code.push_str("    path: \"/\",\n");
    rust_code.push_str("    properties: vec![],\n");
    rust_code.push_str("    children: vec![\n");
    
    // 递归生成子节点
    // Recursively generate child nodes
    generate_node_code(&root, &mut rust_code, 2);
    
    rust_code.push_str("    ],\n");
    rust_code.push_str("};\n");
    
    Ok(rust_code)
}

/// 生成节点代码
/// Generate node code
/// 
/// 递归生成设备树节点的Rust代码
/// Recursively generates Rust code for device tree nodes
fn generate_node_code(node: &DeviceTreeNode, code: &mut String, indent: usize) {
    let indent_str = " ".repeat(indent);
    
    for child in &node.children {
        code.push_str(&format!("{}{{\n", indent_str));
        code.push_str(&format!("{}    name: \"{}\",\n", indent_str, child.name));
        code.push_str(&format!("{}    path: \"{}\",\n", indent_str, child.path));
        
        code.push_str(&format!("{}    properties: vec![\n", indent_str));
        for prop in &child.properties {
            code.push_str(&format!("{}        Property {{\n", indent_str));
            code.push_str(&format!("{}            name: \"{}\",\n", indent_str, prop.name));
            code.push_str(&format!("{}            value: ", indent_str));
            
            match &prop.value {
                PropertyValue::String(s) => {
                    // 确保字符串值被正确引用，避免双重引号
                    let escaped_s = s.replace('"', "\\\"");
                    code.push_str(&format!("PropertyValue::String(\"{}\")\n", escaped_s));
                }
                PropertyValue::Integer(i) => {
                    code.push_str(&format!("PropertyValue::Integer({})\n", i));
                }
                PropertyValue::Boolean(b) => {
                    code.push_str(&format!("PropertyValue::Boolean({})\n", b));
                }
                PropertyValue::IntegerArray(arr) => {
                    code.push_str(&format!("PropertyValue::IntegerArray(vec!["));
                    for (i, num) in arr.iter().enumerate() {
                        if i > 0 {
                            code.push_str(", ");
                        }
                        code.push_str(&num.to_string());
                    }
                    code.push_str("]))\n");
                }
            }
            
            code.push_str(&format!("{}        }},\n", indent_str));
        }
        code.push_str(&format!("{}    ],\n", indent_str));
        
        code.push_str(&format!("{}    children: vec![\n", indent_str));
        generate_node_code(child, code, indent + 4);
        code.push_str(&format!("{}    ],\n", indent_str));
        
        code.push_str(&format!("{}}},\n", indent_str));
    }
}

/// 获取架构特性
/// Get architecture features
/// 
/// 根据开发板配置获取架构特性
/// Gets architecture features based on board configuration
fn get_arch_features(board_name: &str) -> Vec<String> {
    let config_path = find_board_config(board_name);
    
    match config_path {
        Some(path) => {
            if let Ok(content) = fs::read_to_string(&path) {
                let config = parse_toml_config(&content);
                
                // 获取CPU核心信息
                // Get CPU core information
                let default_core = "cortex-m3".to_string();
                let core = config.get("cpu.core").unwrap_or(&default_core);
                
                // 根据核心信息确定架构特性
                // Determine architecture features based on core information
                match core.as_str() {
                    "cortex-m0" | "cortex-m0plus" => vec!["armv6-m".to_string()],
                    "cortex-m3" => vec!["armv7-m".to_string()],
                    "cortex-m4" | "cortex-m7" => vec!["armv7-em".to_string()],
                    "cortex-m23" => vec!["armv8-m-base".to_string()],
                    "cortex-m33" | "cortex-m55" | "cortex-m85" => vec!["armv8-m-main".to_string()],
                    "cortex-a5" | "cortex-a7" | "cortex-a8" | "cortex-a9" | "cortex-a15" => vec!["armv7-a".to_string()],
                    "cortex-a53" | "cortex-a55" | "cortex-a72" | "cortex-a73" | "cortex-a75" | "cortex-a76" | "cortex-a77" | "cortex-a78" => vec!["armv8-a".to_string()],
                    "cortex-a710" | "cortex-a715" | "cortex-a720" | "cortex-a520" => vec!["armv9-a".to_string()],
                    _ => vec!["armv7-m".to_string()], // 默认值
                }
            } else {
                vec!["armv7-m".to_string()] // 默认值
            }
        }
        None => {
            vec!["armv7-m".to_string()] // 默认值
        }
    }
}

/// 获取目标架构
/// Get target architecture
/// 
/// 根据开发板配置获取目标架构
/// Gets target architecture based on board configuration
fn get_target_arch(board_name: &str) -> String {
    let config_path = find_board_config(board_name);
    
    match config_path {
        Some(path) => {
            if let Ok(content) = fs::read_to_string(&path) {
                let config = parse_toml_config(&content);
                
                // 获取CPU核心信息
                // Get CPU core information
                let default_core = "cortex-m3".to_string();
                let default_fpu = "false".to_string();
                let core = config.get("cpu.core").unwrap_or(&default_core);
                let fpu = config.get("cpu.fpu").unwrap_or(&default_fpu) == "true";
                
                // 根据核心信息和FPU支持确定目标架构
                // Determine target architecture based on core information and FPU support
                match core.as_str() {
                    "cortex-m0" | "cortex-m0plus" => "thumbv6m-none-eabi".to_string(),
                    "cortex-m3" => "thumbv7m-none-eabi".to_string(),
                    "cortex-m4" => if fpu { "thumbv7em-none-eabihf".to_string() } else { "thumbv7em-none-eabi".to_string() },
                    "cortex-m7" => if fpu { "thumbv7em-none-eabihf".to_string() } else { "thumbv7em-none-eabi".to_string() },
                    "cortex-m23" => "thumbv8m.base-none-eabi".to_string(),
                    "cortex-m33" => if fpu { "thumbv8m.main-none-eabihf".to_string() } else { "thumbv8m.main-none-eabi".to_string() },
                    "cortex-m55" => if fpu { "thumbv8m.main-none-eabihf".to_string() } else { "thumbv8m.main-none-eabi".to_string() },
                    "cortex-m85" => if fpu { "thumbv8m.main-none-eabihf".to_string() } else { "thumbv8m.main-none-eabi".to_string() },
                    "cortex-a5" => "armv7a-none-eabihf".to_string(),
                    "cortex-a7" => "armv7a-none-eabihf".to_string(),
                    "cortex-a8" => "armv7a-none-eabihf".to_string(),
                    "cortex-a9" => "armv7a-none-eabihf".to_string(),
                    "cortex-a15" => "armv7a-none-eabihf".to_string(),
                    "cortex-a53" => "aarch64-none-elf".to_string(),
                    "cortex-a55" => "aarch64-none-elf".to_string(),
                    "cortex-a72" => "aarch64-none-elf".to_string(),
                    "cortex-a73" => "aarch64-none-elf".to_string(),
                    "cortex-a75" => "aarch64-none-elf".to_string(),
                    "cortex-a76" => "aarch64-none-elf".to_string(),
                    "cortex-a77" => "aarch64-none-elf".to_string(),
                    "cortex-a78" => "aarch64-none-elf".to_string(),
                    _ => "thumbv7m-none-eabi".to_string(), // 默认值
                }
            } else {
                "thumbv7m-none-eabi".to_string() // 默认值
            }
        }
        None => {
            "thumbv7m-none-eabi".to_string() // 默认值
        }
    }
}

/// 构建项目
/// Build project
/// 
/// 构建bootloader或kernel镜像
/// Builds bootloader or kernel images
fn build(board_name: &str, target: &str) {
    println!("Building for board {}...", board_name);
    
    // 首先生成配置
    // Generate configuration first
    generate_config(board_name);
    
    // 确定目标架构
    // Determine target architecture
    let target_arch = get_target_arch(board_name);
    println!("Using target architecture: {}", target_arch);
    
    // 确定架构特性
    // Determine architecture features
    let arch_features = get_arch_features(board_name);
    println!("Using architecture features: {:?}", arch_features);
    
    // 确定构建目标
    // Determine build target
    let build_target = if target == "boot" || target == "Boot" || target == "BOOT" {
        "boot"
    } else if target == "kernel" || target == "Kernel" || target == "KERNEL" {
        "kernel"
    } else {
        "all"
    };
    
    // 构建bootloader
    // Build bootloader
    if build_target == "all" || build_target == "boot" {
        println!("Building bootloader...");
        
        // 构建命令参数
        // Build command arguments
        let mut args = Vec::new();
        args.push("build".to_string());
        args.push("--target".to_string());
        args.push(target_arch.clone());
        for feature in &arch_features {
            args.push(format!("--features={}", feature));
        }
        
        // 转换为 &str 切片
        // Convert to &str slice
        let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        
        let status = Command::new("cargo")
            .args(&args_ref)
            .current_dir("boot")
            .status();
        
        match status {
            Ok(status) if status.success() => println!("Bootloader built successfully!"),
            Ok(_) => println!("Error: Bootloader build failed"),
            Err(e) => println!("Error: Failed to run cargo build: {}", e),
        }
    }
    
    // 构建kernel
    // Build kernel
    if build_target == "all" || build_target == "kernel" {
        println!("Building kernel...");
        
        // 构建命令参数
        // Build command arguments
        let mut args = Vec::new();
        args.push("build".to_string());
        args.push("--target".to_string());
        args.push(target_arch.clone());
        for feature in &arch_features {
            args.push(format!("--features={}", feature));
        }
        
        // 转换为 &str 切片
        // Convert to &str slice
        let args_ref: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        
        let status = Command::new("cargo")
            .args(&args_ref)
            .current_dir("kernel")
            .status();
        
        match status {
            Ok(status) if status.success() => println!("Kernel built successfully!"),
            Ok(_) => println!("Error: Kernel build failed"),
            Err(e) => println!("Error: Failed to run cargo build: {}", e),
        }
    }
}

/// 清理构建
/// Clean build
/// 
/// 清理构建输出和生成的文件
/// Cleans build output and generated files
fn clean_build() {
    println!("Cleaning build...");
    
    // 清理链接脚本
    // Clean linker scripts
    let _ = fs::remove_file("boot/link.x");
    let _ = fs::remove_file("kernel/link.x");
    let _ = fs::remove_file("common/src/generated/devicetree.rs");
    
    // 清理构建输出
    // Clean build output
    let _ = Command::new("cargo")
        .arg("clean")
        .current_dir("boot")
        .status();
    
    let _ = Command::new("cargo")
        .arg("clean")
        .current_dir("kernel")
        .status();
    
    println!("Build cleaned!");
}

/// 解析TOML配置
/// Parse TOML configuration
/// 
/// 解析TOML格式的配置文件
/// Parses TOML format configuration files
fn parse_toml_config(content: &str) -> HashMap<String, String> {
    let mut config = HashMap::new();
    let mut current_section = String::new();
    
    for line in content.lines() {
        let line = line.trim();
        
        // 跳过空行和注释
        // Skip empty lines and comments
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        
        // 检查是否是章节
        // Check if it's a section
        if line.starts_with('[') && line.ends_with(']') {
            current_section = line[1..line.len()-1].to_string();
            continue;
        }
        
        // 解析键值对
        // Parse key-value pairs
        if let Some((key, value)) = parse_key_value(line) {
            let full_key = if current_section.is_empty() {
                key.to_string()
            } else {
                format!("{}.{}", current_section, key)
            };
            config.insert(full_key, value.to_string());
        }
    }
    
    config
}

/// 解析键值对
/// Parse key-value pair
/// 
/// 解析配置文件中的键值对
/// Parses key-value pairs in configuration files
fn parse_key_value(line: &str) -> Option<(&str, &str)> {
    let parts: Vec<&str> = line.splitn(2, '=').collect();
    if parts.len() == 2 {
        let key = parts[0].trim();
        let mut value = parts[1].trim();
        
        // 去除引号
        // Remove quotes
        value = value.trim_matches('"');
        
        // 去除行内注释（# 之后的部分）
        // Remove inline comments (parts after #)
        if let Some(comment_pos) = value.find('#') {
            value = &value[..comment_pos].trim();
        }
        
        Some((key, value))
    } else {
        None
    }
}

/// 解析十六进制或十进制字符串
/// Parse hexadecimal or decimal string
/// 
/// 解析十六进制或十进制格式的数字字符串
/// Parses hexadecimal or decimal format number strings
fn parse_hex_or_decimal(s: &str) -> u32 {
    // 简化解析，不再需要处理下划线
    // Simplified parsing, no longer need to handle underscores
    if s.starts_with("0x") || s.starts_with("0X") {
        u32::from_str_radix(&s[2..], 16).unwrap_or(0)
    } else {
        // 直接解析，配置文件不再使用下划线分隔数字
        // Direct parsing, configuration files no longer use underscores to separate digits
        s.parse().unwrap_or(0)
    }
}

/// 解析十六进制或十进制字符串为 u64
/// Parse hexadecimal or decimal string to u64
/// 
/// 解析十六进制或十进制格式的数字字符串为 u64 类型
/// Parses hexadecimal or decimal format number strings to u64 type
fn parse_hex_or_decimal_u64(s: &str) -> Result<u64, std::num::ParseIntError> {
    if s.starts_with("0x") || s.starts_with("0X") {
        u64::from_str_radix(&s[2..], 16)
    } else {
        s.parse::<u64>()
    }
}

/// 获取架构类型
/// Get architecture type
/// 
/// 根据开发板配置获取架构类型
/// Gets architecture type based on board configuration
fn get_arch_type(board_name: &str) -> String {
    let config_path = find_board_config(board_name);
    
    match config_path {
        Some(path) => {
            if let Ok(content) = fs::read_to_string(&path) {
                let config = parse_toml_config(&content);
                
                // 获取CPU核心信息
                // Get CPU core information
                if let Some(core) = config.get("cpu.core") {
                    if core.contains("cortex") {
                        return "arm".to_string();
                    } else if core.contains("riscv") || core.contains("sifive") {
                        return "riscv".to_string();
                    }
                }
            }
        }
        None => {}
    }
    
    // 默认返回arm
    // Default to arm
    "arm".to_string()
}

/// 生成链接脚本
/// Generate linker script
/// 
/// 根据架构和内存配置生成链接脚本
/// Generates linker script based on architecture and memory configuration
fn generate_linker_script(
    arch_type: &str,
    flash_origin: u32,
    flash_length: u32,
    ram_origin: u32,
    ram_length: u32,
    stack_size: u32,
    heap_size: u32
) -> String {
    // 读取架构特定的链接脚本模板
    // Read architecture-specific linker script template
    let template_path = format!("common/arch/{}/link.x.in", arch_type);
    
    let template_content = match fs::read_to_string(&template_path) {
        Ok(content) => content,
        Err(_) => {
            // 如果模板文件不存在，使用默认模板
            // If template file doesn't exist, use default template
            return format!(r#"/* Linker script - Generated by feathercore-build */
MEMORY
{{
    FLASH (rx) : ORIGIN = 0x{:08X}, LENGTH = {}K
    RAM (rwx) : ORIGIN = 0x{:08X}, LENGTH = {}K
}}

SECTIONS
{{
    .text : {{
        KEEP(*(.vector_table))
        *(.text*)
        *(.rodata*)
    }} > FLASH
    
    .data : {{
        _sdata = .;
        *(.data*)
        _edata = .;
    }} > RAM AT > FLASH
    
    .bss : {{
        _sbss = .;
        *(.bss*)
        *(COMMON)
        _ebss = .;
    }} > RAM
    
    .stack : {{
        . = ALIGN(8);
        _stack_top = .;
        . += {}K;
        _stack_bottom = .;
    }} > RAM
    
    .heap : {{
        . = ALIGN(8);
        _heap_start = .;
        . += {}K;
        _heap_end = .;
    }} > RAM
}}

PROVIDE(_sidata = LOADADDR(.data));
"#,
                flash_origin,
                flash_length / 1024,
                ram_origin,
                ram_length / 1024,
                stack_size / 1024,
                heap_size / 1024
            );
        }
    };
    
    // 替换模板中的变量
    // Replace variables in template
    template_content
        .replace("@FLASH_ORIGIN@", &format!("0x{:08X}", flash_origin))
        .replace("@FLASH_LENGTH@", &format!("{}K", flash_length / 1024))
        .replace("@RAM_ORIGIN@", &format!("0x{:08X}", ram_origin))
        .replace("@RAM_LENGTH@", &format!("{}K", ram_length / 1024))
        .replace("@STACK_SIZE@", &format!("{}K", stack_size / 1024))
        .replace("@HEAP_SIZE@", &format!("{}K", heap_size / 1024))
}
