if exists('g:loaded_tmux_creator')
  finish
endif

let g:loaded_tmux_creator = 1
let s:bindings = function("tmcr#shared#bindings#configure_bindings")

if !executable("tmcr")
  throw "Please install TmuxCreator cli:  https://github.com/Rolf1e/tmux-creator"
endif

call s:bindings()


