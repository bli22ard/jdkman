# JDKMan

[English](README.md)

JDKMan 是一个用 Rust 编写的跨平台 JDK 版本管理器。它允许您轻松管理系统上的多个 JDK 安装。

## 特性

- 添加、删除、列出和激活不同的 JDK 版本
- 跨平台支持（Windows、macOS、Linux）
- 国际化支持（目前支持英语和简体中文）

## 安装

### 预编译二进制文件

您可以从 [Releases](https://github.com/bli22ard/jdkman/releases) 页面下载适用于 Windows、macOS 和 Linux 的预编译二进制文件。

1. 下载适合您操作系统的二进制文件。
2. 解压缩文件（Windows 用 zip，macOS 和 Linux 用 tar.gz）。
3. 将解压后的目录添加到系统的 PATH 环境变量中。

### 从源代码构建

要从源代码构建 JDKMan，您需要在系统上安装 Rust。如果您还没有安装 Rust，可以从 [https://rustup.rs/](https://rustup.rs/) 获取。

安装 Rust 后，请按照以下步骤操作：

1. 克隆仓库：
   ```
   git clone https://github.com/bli22ard/jdkman.git
   cd jdkman
   ```

2. 构建项目：
   ```
   cargo build --release
   ```

3. 编译后的二进制文件将位于 `target/release/jdkman`（Windows 上是 `target/release/jdkman.exe`）。

4. （可选）将二进制文件添加到系统的 PATH 中，以便于访问。

## 使用方法

JDKMan 提供以下命令：

- `jdkman add <name> <path>`: 添加新的 JDK
- `jdkman remove <name>`: 移除已添加的 JDK
- `jdkman list`: 列出所有已添加的 JDK
- `jdkman activate <name>`: 激活指定的 JDK

要获取每个命令的更多信息，请使用 `--help` 选项：