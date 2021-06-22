function! tmcr#shared#bindings#configure_bindings()
  command! -nargs=+ TmuxCreatorLaunchSession call tmcr#shared#open#open_session(<f-args>)
  command! -nargs=+ TmuxCreatorKillSession call tmcr#shared#kill#kill_session(<f-args>)
  command! -nargs=0 TmuxCreatorRegisteredSession call tmcr#shared#list#registered()
  command! -nargs=0 TmuxCreatorListSession call tmcr#shared#list#opened()
endfunction
