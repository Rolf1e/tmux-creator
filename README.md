This is a little term client to manage my tmux config 
I wanted to play a bit with Rust term client making


## What is it ?
It is a small binary to manage tmux sessions. 

## TODO
 - [ ] Add some error check on the session we are killing (I'm pretty sure it's
   bad to kill the session we are lel)
 - [ ] Check if there is a possibility to communicate threw TMUX sockets ` lsof -U | grep '^tmux'`
 - [ ] Installation script
 - [ ] Testing with vader

## Installation 

```bash
  git clone https://github.com/Rolf1e/tmux-creator
  make release && sudo make install
``` 
## Configuration
TmuxCreator works by reading a file at `~/.config/tmux-creator.yml`
```
  # setup up config 
  cd ~/.config
  mkdir tmux-creator
  touch config.yml # you can help yourself with config-sample.yml
``` 

## Binary 
```bash
$tmcr -h

    tmcr [command [args]]

    command [args]:
    -l : list loaded session.
    -r : list all sessions available in config.
    -a {name}: load config from {name}.
    -k {name}: kill tmux session from {name}.
```


## Vim plugin

Available commands

- `:TmuxCreatorRegisteredSession`: list registered sessions
- `:TmuxCreatorListSession`: list running sessions
- `:TmuxCreatorLaunchSession session `: lauch a registered tmux session 
- `:TmuxCreatorKillSession session`: kill a launched session

