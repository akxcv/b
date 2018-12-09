# b

A "bookmark" manager for UNIX.

## Motivation

We run many commands during work. Some of them are run very frequently, others are important for us,
but too long or hard to remember.

We tend to stash these kinds of commands to bash aliases (or, though rarely, functions). But
sometimes we struggle to remember how a particular alias was called, or what it does, and so on.

Aliases and functions are pretty great, but I thought that having a dedicated "bookmark" utility
with a built-in fuzzy finder would be much more convenient. For example, you won't have to deal with
naming aliases (short *or* descriptive?) because you can just use the fuzzy finder.

## Usage

First, install `b`. For now, the only method is to clone this repo and run `cargo run --release`.
Rust >= 1.31.0 is required. Then, symlink `target/release/b` somewhere in your `$PATH`.

Then, add some bookmarks. You can run `b -e` to edit `~/.b`, which is a YAML file where you should
put your bookmarks.

Let's say you ended up with the following file:
```yaml
evil_events: ~/projects/evil_events
eesp: PGPASSWORD=12345 psql postgresql://1.1.1.1:5432/evil_events?user=0exp
sshprod: ssh root@0.1.2.3
killflow: ps aux | grep flow | grep -v grep | awk '{ print $2 }' | xargs kill -9
```

Now, you can use `b` like this:
```sh
$ cd `b evil_events` # expands to `cd /Users/[your name]/projects/evil_events
$ b # will open `b` in fuzzy-finder mode. Press [enter] to use the selected bookmark
$ b eesp # will connect you to your remote postgres database

# Let's say you forgot how your "production ssh" bookmark was called.
# But you remember it had something to do with `prod`:
$ b prod # this will open the fuzzy finder with `prod` as the query

# You can also print a list of bookmarks:
$ b -l
```

## Limitations

- Bookmarks like `pushd $(mktemp -d)` don't work. That would require some serious shell language
parsing.
