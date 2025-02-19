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

## Usage

### First Time Use

Please fill some Mihomosh configs first through the command below:

```sh
mihomosh config edit
```

It would open the editor based on `EDITOR` environment or `vim` as fallback, if you want to use other editor (such as `nano`), please use option `-e`:

```sh
mihomosh config edit -e nano
```

If you want to view your Mihomosh configs, please: **(relying on `less` command)**

```sh
mihomosh config view
```

If you want to reset all of the Mihomo configs, please:

```sh
mihomosh config reset
```

### Profile Management

Help is all you need, eveything is so trivial

```sh
mihomosh profile help
```

### URL Testing

Use command below:

```sh
mihomosh test <URL>
```

### Inspect Activated Profile

Help is all you need, eveything is so trivial

```sh
mihomosh show help
```

### Inspect Mihomo Status

Help is all you need, eveything is so trivial

```sh
mihomosh status help
```

### Mihomo Controlling

Help is all you need, eveything is so trivial

```sh
mihomosh ctrl help
```

## Build

```sh
cargo build --release
```
