% TM(1) Version 0.1.0 | Tmux "Manager" Documentation

NAME
====

`tm` — Tmux "_session_/_window_" management

SYNOPSIS
========

| `tm` \[`-h`|`--help`] \[`-v`|`--version`] \[`subcommand`]

DESCRIPTION
===========

Todo, add _description_.

Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod
tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim
veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea
commodo consequat. Duis aute irure dolor in reprehenderit in voluptate
velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat
cupidatat non proident, sunt in culpa qui officia deserunt mollit anim
id est laborum.

1. Sed ut perspiciatis unde omnis iste natus error sit
2. Sed ut perspiciatis unde omnis iste natus error sit
3. Sed ut perspiciatis unde omnis iste natus error sit
4. Sed ut perspiciatis unde omnis iste natus error sit

Global Options
==============

`-h`, `--help`
:   Print help information.

`-v`, `--version`
:   Print version information.

SUBCOMMAND
==========

add
---

Register a path to use when listing paths to attach.

`<path>...`
:   Optional paths to be added. Uses `cwd` if not present.

`-g`, `--global`
:   Save to global `$XDG_CONFIG_HOME` instead of `$XDG_DATA_HOME`.

`-l`, `--long`
:   List current paths.

`-w`, `--workspace`
:   Use path as a workspace path.

`-h`, `--help`
:   Print help information.

attach
------

Create or attach to a tmux session based on the path specified.

`<query>...`
:   Query to search from.

`-e`, `--exist`
:   Attach to existing tmux session.

`-p`, `--path` \<path>
:   Exact path to create or attach tmux session.

`-x`, `--exact`
:   Use exact match search.

`-h`, `--help`
:   Print help information.

config
------

Get or set configuration options.

`<name>`
:   Name of configuration option.

`<value>`
:   Value of the configuration option defined by name.

`-e`, `--edit`
:   Open config file in `$EDITOR`.

`-g`, `--global`
:   Save to global `$XDG_CONFIG_HOME` instead of `$XDG_DATA_HOME`.

`-l`, `--list`
:   List all config options and values.

`--options`
:   List all config options with a description of each.

`-h`, `--help`
:   Print help information.

jump
----

Store a list of paths and jump to that index. This is useful for keybindings where you set
keybindingd to jump to index 1, 2, 3, ... and tm will check the list of stored paths and use that to
jump to that tmux session.

By default if no options are passed then the cwd is added to the jump list.

`-e`, `--edit`
:   Open config file in `$EDITOR`.

`-i`, `--index` `<index>`
:   Jump to index in jump list. Index is `1` based.

`-l`, `--list`
:   Output jumplist.

`-p`, `--path` `<path>`
:   Add path to jump list.

`-h`, `--help`
:   Print help information.

kill
----

Kill a running tmux session.

`<query>...`
:   Query to search from.

`-a`, `--all`
:   Kill all sessions.

`-x`, `--exact`
:   Use exact match search.

`-h`, `--help`
:   Print help information.

list
----

List current sessions.

`-h`, `--help`
:   Print help information.

path
----

Manage registered *single* and *workspace* paths.

`add`

Register a path to use when listing paths to attach.

`<path>...`
:   Optional paths to be added. Uses `cwd` if not present.

`-g`, `--global`
:   Save to global `$XDG_CONFIG_HOME` instead of `$XDG_DATA_HOME`.

`-l`, `--long`
:   List current paths.

`-w`, `--workspace`
:   Use path as a workspace path.

`-h`, `--help`
:   Print help information.

`remove`

Remove registered path from tm.

`-g`, `--global`
:   Remove from global config.

`-w`, `--workspace`
:   Remove only workspace paths.

`-x`, `--exact`
:   Use exact match search.

`-h`, `--help`
:   Print help information

wcmd
----

Send a command to a execute in a tmux window.

`<window>`
:   Name of the window to execute the command from. This name window name can be taken from
    a path. In this case the basename will be used. This is useful with git worktrees and
    different branches.

`<cmd>...`
:   The command to be executed in the tmux window. Passing this after `--` will make sure
    that no option parsing is completed and the entire command is sent to the tmux window.
    This however does not have to be after `--`.

`-h`, `--help`
:   Print help information

CONFIGURATION
=============

`depth`: *number*
:   Limit directory traversal for *workspace* paths. By default, the depth limit is set to 10.

`height`: *number*
:   Set the height of fuzzy selector, as a percentage of the terminal. By default, value is 40 (%).

`single_paths`: *list*
:   List of paths to be added to tm's search list. These paths are appended after the workspace
    paths are searched.

`workspace_paths`: *list*
:   List of paths to be searched for valid git repositories. Every valid git repository will be
    added to the list


ENVIRONMENT
===========

**TM_CONFIG_PATH**

:   Override the location of the `global` config path. If not defined, the `global` config path will
    be set by `$XDG_CONFIG_HOME/tm`. If this is also not present, will fallback to:

    - `$HOME/.config/tm` on `linux`
    - `$HOME/Library/Application Support/tm` on `macos`.


**TM_DATA_PATH**

:   Override the location of the `local` config path. If not defined, the `local` config path will
    be set by `$XDG_DATA_HOME/tm`. If this is also not present, will fallback to:

    - `$HOME/.local/share/tm` on `linux`
    - `$HOME/Library/Application Support/tm` on `macos`.

FILES
=====

*config.toml*

:   Configuration file

*jumplist*

:   File containing paths in jumplist

SHELL COMPLETION
================

Shell completion files are included in the release tarball for `Bash`, `Fish`, `Zsh` and `PowerShell`.

For `bash`, move tm.bash to `$XDG_CONFIG_HOME/bash_completion` or `/etc/bash_completion.d/`.

For `fish`, move tm.fish to `$HOME/.config/fish/completions`.

For `zsh`, move _tm to one of your `$fpath` directories.

BUGS
====

See GitHub Issues: <https://github.com/EdenEast/tuxmux/issues>
