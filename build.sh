#!/bin/bash
# FeatherCore OS Build Script
# 构建 FeatherCore 操作系统

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FEATHERCORE_ROOT="$SCRIPT_DIR"

# Colors for output
# 输出颜色
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

echo_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

echo_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Build build_tool
# 编译构建工具
build_build_tool() {
    echo_info "Building build tool..."
    cd "$FEATHERCORE_ROOT/build_tool"
    cargo build --release
    echo_info "Build tool built successfully"
}

# List supported boards
# 列出支持的开发板
list_boards() {
    echo_info "Supported boards:"
    "$FEATHERCORE_ROOT/build_tool/target/release/feathercore-build" \
        -r "$FEATHERCORE_ROOT" list-boards
}

# Generate configuration for a board
# 生成开发板配置
generate_config() {
    local board="$1"
    
    if [ -z "$board" ]; then
        echo_error "Board name not specified"
        echo "Usage: $0 generate <board-name>"
        exit 1
    fi
    
    echo_info "Generating configuration for $board..."
    "$FEATHERCORE_ROOT/build_tool/target/release/feathercore-build" \
        -r "$FEATHERCORE_ROOT" generate "$board"
    echo_info "Configuration generated successfully"
}

# Build boot image
# 编译 boot 镜像
build_boot() {
    local board="$1"
    local target="$2"
    
    if [ -z "$board" ] || [ -z "$target" ]; then
        echo_error "Board name or target not specified"
        echo "Usage: $0 build-boot <board-name> <target>"
        exit 1
    fi
    
    echo_info "Building boot image for $board ($target)..."
    cd "$FEATHERCORE_ROOT/boot"
    cargo build --release --features "$board" --target "$target"
    echo_info "Boot image built successfully"
}

# Build kernel image
# 编译 kernel 镜像
build_kernel() {
    local board="$1"
    local target="$2"
    
    if [ -z "$board" ] || [ -z "$target" ]; then
        echo_error "Board name or target not specified"
        echo "Usage: $0 build-kernel <board-name> <target>"
        exit 1
    fi
    
    echo_info "Building kernel image for $board ($target)..."
    cd "$FEATHERCORE_ROOT/kernel"
    cargo build --release --features "$board" --target "$target"
    echo_info "Kernel image built successfully"
}

# Build rootfs utilities
# 编译 rootfs 工具
build_rootfs() {
    echo_info "Building rootfs utilities..."
    cd "$FEATHERCORE_ROOT/rootfs"
    cargo build --release --features "shell,coreutils" --target "$1"
    echo_info "Rootfs utilities built successfully"
}

# Clean build artifacts
# 清理构建产物
clean() {
    echo_info "Cleaning build artifacts..."
    
    cd "$FEATHERCORE_ROOT/build_tool"
    cargo clean
    
    cd "$FEATHERCORE_ROOT/boot"
    cargo clean
    
    cd "$FEATHERCORE_ROOT/kernel"
    cargo clean
    
    cd "$FEATHERCORE_ROOT/rootfs"
    cargo clean
    
    echo_info "Build artifacts cleaned"
}

# Full build
# 完整构建
build_all() {
    local board="$1"
    local target="$2"
    
    if [ -z "$board" ] || [ -z "$target" ]; then
        echo_error "Board name or target not specified"
        echo "Usage: $0 build-all <board-name> <target>"
        echo ""
        echo "Example:"
        echo "  $0 build-all stm32f429i-disc thumbv7em-none-eabihf"
        exit 1
    fi
    
    echo_info "Starting full build for $board ($target)..."
    
    # Step 1: Build build tool
    # 步骤 1: 编译构建工具
    build_build_tool
    
    # Step 2: Generate configuration
    # 步骤 2: 生成配置
    generate_config "$board"
    
    # Step 3: Build boot
    # 步骤 3: 编译 boot
    build_boot "$board" "$target"
    
    # Step 4: Build kernel
    # 步骤 4: 编译 kernel
    build_kernel "$board" "$target"
    
    # Step 5: Build rootfs (optional)
    # 步骤 5: 编译 rootfs (可选)
    # build_rootfs "$target"
    
    echo_info ""
    echo_info "========================================="
    echo_info "Full build completed successfully!"
    echo_info "========================================="
    echo_info ""
    echo_info "Output files:"
    echo_info "  Boot:  $FEATHERCORE_ROOT/boot/target/$target/release/feathercore-boot"
    echo_info "  Kernel: $FEATHERCORE_ROOT/kernel/target/$target/release/feathercore-kernel"
    echo_info ""
}

# Show help
# 显示帮助
show_help() {
    echo "FeatherCore OS Build Script"
    echo "FeatherCore 操作系统构建脚本"
    echo ""
    echo "Usage / 用法:"
    echo "  $0 <command> [options]"
    echo ""
    echo "Commands / 命令:"
    echo "  build-tool          Build the build tool / 编译构建工具"
    echo "  list-boards         List supported boards / 列出支持的开发板"
    echo "  generate <board>    Generate configuration for board / 生成开发板配置"
    echo "  build-boot <board> <target>    Build boot image / 编译 boot 镜像"
    echo "  build-kernel <board> <target>  Build kernel image / 编译 kernel 镜像"
    echo "  build-all <board> <target>     Full build / 完整构建"
    echo "  clean               Clean build artifacts / 清理构建产物"
    echo "  help                Show this help / 显示帮助"
    echo ""
    echo "Examples / 示例:"
    echo "  $0 build-all stm32f429i-disc thumbv7em-none-eabihf"
    echo "  $0 build-kernel stm32n6570-dk thumbv8m.main-none-eabi"
    echo "  $0 list-boards"
    echo ""
}

# Main
# 主程序
case "$1" in
    build-tool)
        build_build_tool
        ;;
    list-boards)
        list_boards
        ;;
    generate)
        generate_config "$2"
        ;;
    build-boot)
        build_boot "$2" "$3"
        ;;
    build-kernel)
        build_kernel "$2" "$3"
        ;;
    build-rootfs)
        build_rootfs "$2"
        ;;
    build-all)
        build_all "$2" "$3"
        ;;
    clean)
        clean
        ;;
    help|--help|-h)
        show_help
        ;;
    *)
        echo_error "Unknown command: $1"
        echo ""
        show_help
        exit 1
        ;;
esac

exit 0
