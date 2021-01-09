<div align="center">
<img src="assets/block.svg" alt="Wagon" width="180" height="160" />

# Damn Vulnerable Blockchain

![CI (Linux)](<https://github.com/realaravinth/damn-vuln-blockchain/workflows/CI%20(Linux)/badge.svg>)
[![Documentation](https://img.shields.io/badge/Docs-master-blue)](https://realaravinth.github.io/damn-vuln-blockchain/damn_vuln_blockchain/index.html)
[![codecov](https://codecov.io/gh/realaravinth/damn-vuln-blockchain/branch/master/graph/badge.svg?token=ZgkisU6TWX)](https://codecov.io/gh/realaravinth/damn-vuln-blockchain)
[![AGPL License](https://img.shields.io/badge/license-AGPL-blue.svg)](http://www.gnu.org/licenses/agpl-3.0)
[![dependency status](https://deps.rs/repo/github/realaravinth/damn-vuln-blockchain/status.svg)](https://deps.rs/repo/github/realaravinth/damn-vuln-blockchain)

</div>

This is a test blockchain that I build for fun and as the name
suggests, **it is bloody vulnerable.**

### How to build

- Install Cargo using [rustup](https://rustup.rs/) with:

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- Clone the repository with:

```
$ git clone https://github.com/realaravinth/damn-vuln-blockchain
```

- Build with Cargo:

```
$ cd damn-vuln-blockchain && cargo build
```

### Usage:

`Damn Vulnerable Blockchain` comes with a peer implementation called
`dwb`. `dwb` supports three modes:

#### - Attacker:

Configured to fork the blockchain and perform a double spend. See
[attack scenario](#attack-scenario%3A).

#### - Auditor:

This is a special peer that acts as the discovery node and
mint. It should be spawned first.

#### - Victim:

This peer will be configured to take itself down when an attack command
is issued.


#### `dwb` usage:

```
Damn Vulnerable Blockchain 0.1
Aravinth Manivannan <realaravinth@batsense.net>
A bloody vulnerable blockchain implementation

USAGE:
    dwb --discovery <discovery> --mode <mode> --id <peer_id> --port <port>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --discovery <discovery>    address of discovery node
    -m, --mode <mode>              available modes:
                                   	auditor
                                   	attacker
                                   	victim
    -i, --id <peer_id>             set peer ID
    -p, --port <port>              set port to listen on

```

### Attack Scenario:

TODO

### Credits:

Logo made by [Freepik](https://www.flaticon.com/authors/freepik) from
[Flaticon](https://www.flaticon.com). Do check them out!
