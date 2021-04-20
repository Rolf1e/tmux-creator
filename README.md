This is a little term client to manage my tmux config 
I wanted to play a bit with Rust term client making


## TODO
 - [ ] Add some error check on the session we are killing (I'm pretty sure it's
   bad to kill the session we are lel)
 - [ ] Check if there is a possibility to communicate threw TMUX sockets ` lsof -U | grep '^tmux'`
 - [ ] Installation script
 - [ ] Testing with vader

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


