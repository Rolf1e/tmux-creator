This is a little term client to manage my tmux config 
I wanted to play a bit with Rust term client making


## TODO
 - [ ] Check if there is a possibility to communicate threw TMUX sockets ` lsof -U | grep '^tmux'`

## Installation 

```bash
  git clone https://github.com/Rolf1e/tmux-creator
  cd tmux-creator
  cargo build --release
  
  # in your .bash_aliases
  alias tmcr="/path/to/project/tmux-creator/target/release/tmux-executor"

  # setup up config 
  cd ~/.config
  mkdir tmux-creator
  touch config.yml # you can help yourself with config-sample.yml
  
``` 


