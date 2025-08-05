# Mihomosh

English | [简体中文](./README-zh_cn.md)

A CLI Toolkit for Mihomo

> [!IMPORTANT]
> Mihomosh **NOT INCLUDES** Mihomo itself  
> Mihomosh **IS ONLY** a toolkit for Mihomo  
> If you have not installed Mihomo on your machine, please refer to [documents](https://wiki.metacubex.one/) for installing

> [!CAUTION]
> v2 is completely not compatible with v1 due to the broken changes  
> Please [migrate](./README-zh_cn.md#版本迁移) for better expierence

## Motivation

- Currently, most third-party Mihomo tools/clients are GUI-based, which is not friendly for server environments without a GUI.
- Although web-based panels can be used, this means exposing server ports, which can lead to security issues. Even if you can wrap the panel with an SSH tunnel to avoid direct port exposure, setting up the tunnel is also troublesome.
- Of course, there are some command-line-based Mihomo tools available, but they are mostly all-in-one solutions with limited operability and flexibility.
- I need to run Mihomo in isolated Docker containers, which can only be operated through an external control API. Existing command-line tools generally do not support it at all.

## Features

- Supports subscription management (create, delete, edit, update, view, extend configuration/script, activate)
- Supports proxy selection
- Supports proxy set management (view, update, latency test)
- Supports rule set management (view, update)
- Supports viewing real-time logs/traffic/memory usage
- Supports viewing and closing current connections
- Supports command auto-completion
- Automatically reactivates upon subscription changes to take effect

Mihomosh allows almost all functions provided by the Mihomo external control API to be operated without using complex cURL commands.

## Prerequisites

- **Must** have Mihomo installed on your machine and the external control API enabled.
- (Optional) By default, `nano` is used as the file editor, but you can specify a different file editor via parameters.
- (Optional) By default, `less` is used as the file browser, but you can specify a different file browser via parameters.

## Installation

### Quick Installation (for Unix-like systems)

Automatically searches GitHub Releases for a precompiled binary file matching your system to download and install.

```sh
$ curl https://i.jpillora.com/SamuNatsu/mihomosh@latest! | bash
```

### Manual Installation

Latest version: <https://github.com/SamuNatsu/mihomosh/releases/latest>  
Please find and download the precompiled binary file matching your system and architecture.

We provide precompiled binaries for:

- Linux (i686, x86_64, arm64)
- MacOS (x86_64, arm64)
- Windows (i686, x86_64, arm64)

### Cargo Compilation Installation

If the precompiled binary does not meet your needs, you can compile it using Cargo.

```sh
$ cargo install --git https://github.com/SamuNatsu/mihomosh.git
```

---

_English version of document is working in progress_  
_You can check out the [simplified Chinese](./README-zh_cn.md) version for more_
