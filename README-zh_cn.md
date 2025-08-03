# Mihomosh

[English](./README.md) | 简体中文

一个 Mihomo 命令行工具

> [!IMPORTANT]
> Mihomosh **不包含** Mihomo 本体  
> Mihomosh **只是** 一个用于操作 Mihomo 的工具  
> 如果你的机器没有安装 Mihomo，请参考[文档](https://wiki.metacubex.one/)进行安装

> [!CAUTION]
> v2 版本因为添加了很多新功能和修改，导致与 v1 版本的配置完全不兼容！  
> 请进行[迁移](#版本迁移)以体验最新功能

## 制作动机

- 目前大部分 Mihomo 第三方工具/客户端都是基于 GUI 的，这对于没有 GUI 环境的服务器环境来说很不友好
- 虽然还有基于 Web 的面板可以使用，但是这意味着你需要暴露服务器端口，容易产生安全问题以及被查水表，即使你可以给面板套上一层 SSH 隧道来避免直接暴露端口，但开启隧道同样也是麻烦事
- 当然目前也有一些基于命令行的 Mihomo 工具，但是它们基本上是一站式操作，可操作性和自由度有限
- 我需要把 Mihomo 运行在隔离的 Docker 容器中，只能通过外部控制 API 进行操作，而现有的命令行工具基本上难以做到简便快捷，甚至完全就不支持

## 特性

- 支持订阅管理（创建、删除、编辑、更新、查看、扩展配置/脚本、激活）
- 支持代理选择
- 支持代理组管理（查看、更新、延迟测试）
- 支持规则组管理（查看、更新）
- 支持查看实时日志/流量/内存占用
- 支持查看和关闭当前连接
- 支持命令自动补全
- 在订阅发生变化时自动重新激活以生效

Mihomo 外部控制 API 所提供的功能几乎都可以通过 Mihomosh 进行操作，而不需要使用复杂的 cURL 命令

## 先决条件

- 你的机器上 **必须** 安装了 Mihomo，且开启了外部控制 API
- _（可选）_ 默认使用 `nano` 作为文件编辑器，不过你可以传入参数自行选择文件编辑器
- _（可选）_ 默认使用 `less` 作为文件浏览器，不过你可以传入参数自行选择文件浏览器

## 安装

### 快速安装（适用于类 Unix 系统）

自动从 GitHub Release 中搜索与你系统相匹配的预编译二进制文件进行下载安装

```sh
$ curl https://i.jpillora.com/SamuNatsu/mihomosh@latest! | bash
```

### 手动安装

最新版本：<https://github.com/SamuNatsu/mihomosh/releases/latest>  
请寻找与你系统和架构相匹配的预编译二进制文件下载安装

我们提供如下预编译二进制：

- Linux（i686、x86_64、arm64）
- MacOS（x86_64、arm64）
- Windows（i687、x86_64、arm64）

### Cargo 编译安装

如果预编译二进制中没有你需要的，你可以通过 Cargo 编译安装

```sh
$ cargo install --git https://github.com/SamuNatsu/mihomosh.git
```

## 初次使用须知

**如果你的 Mihomo 配置文件路径需要使用 root 用户才能进行写入，那么你同样应该使用 root 用户来运行 Mihomosh，否则可能出现权限问题！**

初次使用时，你需要编辑 Mihomosh 配置，告诉它 Mihomo 的配置文件路径以及 API 链接：

```sh
$ mihomosh config edit
```

默认会打开 `nano` 作为编辑器，如果你需要使用其他编辑器，请携带参数 `-e`，如：

```sh
$ mihomosh config edit -e vim
```

在编辑器中，你需要修改 `mihomo-path`、`mihomo-api` 和 `mihomo-secret` 为对应的配置文件路径、外部控制 API 和 API token（可选）

## 版本迁移

由于 v2 为了实现新功能而修改了很多文件配置和结构，导致与 v1 无法兼容

请将以下文件夹的内容（v1 配置）拷贝并删除：

- Linux：`~/.local/share/mihomosh`
- MacOS: `~/Library/Application Support/io.github.SNRainiar.mihomosh`
- Windows: `~\AppData\Local\SNRainiar\mihomosh\data`

在 v2 中，会覆盖使用以下文件夹保存配置：

- Linux：`~/.local/share/mihomosh`
- MacOS: `~/Library/Application Support/io.github.SamuNatsu.mihomosh`
- Windows: `~\AppData\Local\SamuNatsu\mihomosh\data`

你需要迁移 v1 文件夹中的如下内容：

1. 主配置文件 `config.yaml`
2. 订阅配置文件夹 `profile_conf` 中所有的文件

## 用法

所有命令的用法都可以通过 `-h` 或 `--help` 参数获得详细的说明，因此这里只会对一些需要特别注意的操作进行讲解：

```txt
Usage: mihomosh <COMMAND>

Commands:
  config            Manage configs
  profile           Manage profiles
  connection        Manage connections
  inspect           Inspect Mihomo runtime info
  control           Control Mihomo
  proxy             Print/Update/Test proxies
  proxy-set         Print/Update/Test proxy sets
  rule              Print rules
  rule-set          Print/Update rule sets
  shell-completion  Generate shell completion
  help              Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### 使用命令行补全

Mihomosh 支持 `bash`、`elvish`、`fish`、`powershell` 和 `zsh` 的命令行补全：

```txt
Usage: mihomosh shell-completion <SHELL>

Arguments:
  <SHELL>  Target shell name [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -h, --help  Print help
```

执行命令后，mihomosh 会输出对应 Shell 的补全脚本，请参考对应 Shell 的文档来安装这个脚本

例如 Bash，你需要在 `.bashrc` 中添加以下代码：

```sh
eval "$(mihomosh shell-completion bash)"
```

### 代理组选择代理

有些代理组可以手动选择使用的代理（如 GLOBAL 代理组选择使用的全局代理），你可以使用如下命令：

```sh
$ mihomosh proxy update
```
