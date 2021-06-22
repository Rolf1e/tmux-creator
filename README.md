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
 - [ ] Better documentation with screenshots


# Binary 
## Installation 
```bash
  git clone https://github.com/Rolf1e/tmux-creator
  make install
``` 
## Configuration
TmuxCreator works by reading a file at `~/.config/tmux-creator.yml`
```
  # setup up config 
  touch ~/.config/tmux-creator.yml # you can help yourself with config-sample.yml
``` 
## Usage
```bash
$tmcr -h

    tmcr [command [args]]

    command [args]:
    -l : list loaded session.
    -r : list all sessions available in config.
    -a {name}: load config from {name}.
    -k {name}: kill tmux session from {name}.
```


#Vim plugin
## Installation 
You can install with [vim-plug](https://github.com/junegunn/vim-plug) for now. (Still WIP)

`Plug 'Rolf1e/tmux-creator', {'rtp': 'vim'}`

Please make sure that the tmcr binary is installed !

## Usage
- `:TmuxCreatorRegisteredSession`: list registered sessions
- `:TmuxCreatorListSession`: list running sessions
- `:TmuxCreatorLaunchSession session `: lauch a registered tmux session 
- `:TmuxCreatorKillSession session`: kill a launched session

## Picker
Integration with [FZF](https://github.com/junegunn/fzf.vim) !
If fzf is already installed, it will be choose as default picker !

# TODO native picker not working now

