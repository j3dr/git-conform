<p align="left">
  <a href="https://crates.io/crates/git-conform">
    <img alt="Crates.io Version" src="https://img.shields.io/crates/v/git-conform" />
    <img alt="Crates.io Downloads" src=https://img.shields.io/crates/d/git-conform />
  </a>
</p>

## About
`git-conform` is a simple CLI tool that helps you to keep track of the repositories on your local machine and their remote counterparts.
It works by scanning your **home directory** with `gitcf scan --all` (or just the ones you specified) in search for git repositories, and then storing their paths in
the tracking file located at `~/.local/share/gitcf`. By typing `gitcf check --all` you can see useful information
about all the repositories on your machine at once such as uncommitted changes or unsynced commits between local and remote branches.

## Installation

### Through cargo
`cargo install git-conform`

### Using the setup shell script
> [!IMPORTANT]  
> Make sure you have `wget` and `curl` installed on your system
```bash
bash <(curl -sSL https://raw.githubusercontent.com/j3dr/git-conform/main/setup.sh)
```

## Available commands and options
- `gitcf scan` - searches for untracked repositories and automatically adds them for tracking
  - `-a, --all` - scan all directories in your /home
  - `--hidden` - allow scanning hidden directories
  - `-q, --quiet` - suppress information messages
<br></br>
- `gitcf list` - prints the list of tracked repositories
  - `-i, --ignored` - print the list of ignored repositories instead
<br></br>
- `gitcf add` - adds specified repositories for tracking
<br></br>
- `gitcf rm` - removes specified repositories from tracking
  - `-a, --all` - remove all repositories from tracking
<br></br>
- `gitcf ignore` - blacklists specified repositories from being added for tracking by the `scan` command
<br></br>
- `gitcf notice` - removes specified repositories from the blacklist
  - `-a, --all` - remove all repositories from the blacklist
<br></br>
- `gitcf check` - inspects specified repositories
  - `-a, --all` - inspect all tracked repositories
  - `-s, --status` - print only the output of `git status -s`
  - `-r, --remotes` - print only the differences between local repositories and their remotes
<br></br>

Type `gitcf help` or `gitcf -h` to get more details. Also, you can get more comprehensive info about specific subcommand like this: `gitcf help <subcommand>`.
