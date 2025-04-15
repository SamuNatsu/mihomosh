# Mihomosh

A CLI Toolkit for Mihomo

> [!IMPORTANT]
> Mihomosh **DOES NOT** contain Mihomo distribution itself!  
> It **IS** just a toolkit that works with Mihomo!  
> If there is no Mihomo deployed on your machine, please deploy it first.

## Features

- Full featured remote/local profiles management
- Global-level/Profile-level extend configs and extend scripts support
- Basic but enough Mihomo controlling through RESTful API
- Basic Mihomo running status display
- Manual URL testing for activated profile

## Prerequisite

- Mihomosh relies on `vim` for fallback editing
- Mihomosh relies on `less` for showing some very long text

## Install

### Oneline Command (Unix-like ONLY)

```sh
curl -s https://installer.samunatsu.workers.dev/SamuNatsu/mihomosh | bash
```

### Cargo Install

Build and install Mihomosh locally

```sh
cargo install --git https://github.com/SamuNatsu/mihomosh.git
```

### Download from Releases

Latest version: <https://github.com/SamuNatsu/mihomosh/releases/latest>

Please find the `.tar.gz`/`.zip` asset that fits your platform and architecture.

## Best Practices

### 1. If your Mihomo data directory needs root to access, please always run Mihomosh as root user (For Unix-like)

Mihomosh stores configs and data files under user directories (`/home/some_user` for non-root user, `/root` for root user).

If your Mihomo data directory needs root, but your Mihomosh configs are under non-root user directories, it would cause:
1. Permission denied (run Mihomosh as **non-root user**, you are not able to write the Mihomo configs for activating profiles)
2. File not found (run Mihomosh as **root user**, you are not able to find Mihomosh configs under root directories)

Therefore, you'd better always run Mihomosh as root user, Mihomosh configs and data files will be stored under root directories.  
Neither permission denied nor file not found would happen.

If you've already stored your Mihomosh configs and data files under non-root user directories, you can find the folder in `/home/some_user/.local/share/mihomosh`, just move it to `/root/.local/share/mihomosh`, then change the owner and group.

## Usage

See [wiki](https://github.com/SamuNatsu/mihomosh/wiki). **(WIP)**
