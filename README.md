# ASS
Advanced Storage System - a command line tool for managing passwords.

## Building from source
To build **ASS** from source:

1) Ensure that you have [cargo](https://crates.io/) installed on your machine. If not, [follow this official installation guide](https://doc.rust-lang.org/cargo/getting-started/installation.html).
2) Clone this repository.
3) In the root directory run `cargo build` to compile **ASS**.
4) If no errors occured during compilation directory named `target` should appear. In `./target/debug` you will find the compiled binary named `ass` by default.

## Basics of ASS

### Resources

Key-value pairs that represent a stored password. As an example there can be a resource with the **name** `lectoria_api_token` and **value** of `ligma_balls_security`.

### Groups

Groups contain resources and define access level (from 0 to 100) required to access the resources. As an example all resources of group `backend` with required access level being 50 can only be accessed by users with access level of 50 or higher.

### Roles

Roles are assigned to users and define their access level (from 0 to 100). Default roles that are created on setup are:

| Role | Access Level | ID (used in config) |
| -- | -- | -- |
| `admin` | 100 | 1 |
| `mod` | 90 | 2 |
| `user` | 10 | 3 |
| `no` | 0 | 4 |

First registered user on setup is assigned the `admin` role, all other users registered on further runs automatically receive the `user` role.

### Encryption key

Any string can be used as an encryption key. If you are using a UNIX-based OS you can use the following command to generate an encryption key:

```
head -c 128 /dev/urandom | LC_ALL=C tr -dc '[:print:]' > key.txt
```

## Usage

### Preparing the configuration file

Before starting **ASS** you'll have to create a file named `config.cfg` alongside the executable. An example config file is available [here](config.cfg). List of the fields that have to be present in config file:

| Field name | Description | Recommended value |
| -- | -- | -- |
| `DATABASE_PATH` | Path to the file containing the main SQLite database. If the file is not present it will be created during setup. Set the path in the format that's used by your OS. | Any path (e.g. on a thumb drive) |
| `KEY_PATH` | Path to the encryption key. Set the path in the format that's used by your OS. | Any path (e.g. on a thumb drive) |
| `DEFAULT_ROLE_ID` | Specifies the role that will be assigned to users by default after registration. | `3` (equals to default role being `user`) |
| `ADMIN_ACCESS_LEVEL` | Default access level for the `admin` role. | `100` (access level required to access admin-level commands) |
| `MOD_ACCESS_LEVEL` | Default access level for the `mod` role. | `90` (access level required to access moderator-level commands) |

### First start (Setup)

When first starting **ASS** use the `init` flag like so:

```
./ass init
```

**This step is mandatory to ensure that database schemas are created properly.**

### Using ASS

For further runs run `ass` without any additional parameters to enter interactive CLI, use `!q` to exit. From there you will be able to carry out all the supported operations. Most of the operations utilize interactive mode. Full lists of operations:

#### Managing resources

| Command | Description | Usage | Required access level |
| -- | -- | -- | -- |
| new | Creates new resource | `new` or `new <resource name>` | 90 (`mod`) |
| get | Retrieve password from resource | `get <resource name>` | N/A, depends on resource |
| update | Update password of resource | `update` or `update <resource name>` | 90 (`mod`) |
| delete | Delete resource (irreversible) | `delete` or `delete <resource name>` | 100 (`admin`) |

#### Managing resource groups

| Command | Description | Usage | Required role (or higher) |
| -- | -- | -- | -- |
| group | Adds / removes resources from groups | `group` | 100 (`admin`) |
| groups | Lists all available groups | `groups` | 90 (`mod`) |
| ng | Creates a new group | `ng` or `ng <new group name>` | 100 (`admin`) |

#### Managing user roles

| Command | Description | Usage | Required role (or higher) |
| -- | -- | -- | -- |
| role | Assigns / removes roles from users | `role` | 100 (`admin`) |
| roles | Lists all available roles | `roles` | 90 (`mod`) |
| nr | Creates a new role | `nr` or `nr <new role name>` | 100 (`admin`) |

## Feedback

If you found any bugs or experience any issues feel free to send feedback here:
https://feedback.ass.gfxv.cumlord.ru/

![chipi chipi chapa chapa meme in standard definition](https://storage.rferee.dev/assets/media/gifs/chipi-chipi-chapa-chapa-sd.gif)

## Waiver

By engaging with this software, the user (hereinafter referred to as 'You') hereby irrevocably waives all rights, titles, and interests in and to their soul. The developer, gfxv (hereinafter referred to as 'Developer'), is granted full and unrestricted rights to the user's soul, including but not limited to the following purposes:

- The authority to sell or transfer the User's soul to devils, demons, or other supernatural entities in exchange for A1 proficiency in the French language.
- The right to negotiate the sale of the User's soul to Blizzard Entertainment as compensation for access to the latest Overwatch battle pass.
- Utilization of the User's soul as a bargaining chip to facilitate the Developer's escape from infernal realms, a necessity arising from coding in the Rust programming language.
- Permission to trade the User's soul with `rferee`, thereby fulfilling their requirement for new recruits ensnared by Java and Docker, thus sparing the Developer from such a fate.
- Consent for the Developer to operate an identical replication of the User's consciousness within a Docker containerized environment.
- The use of the User's soul as a computational device, specifically to execute a fork bomb, thereby initiating a process of exponential multiplication until system resources are fully exhausted.

Additionally, please note that the Developer is not responsible for any damages to the User's soul, including but not limited to: eternal damnation, demonic possession, and/or loss of sanity.

This agreement is in effect upon the User's initiation of software usage and remains valid indefinitely. Good luck, and have fun (while you still can).