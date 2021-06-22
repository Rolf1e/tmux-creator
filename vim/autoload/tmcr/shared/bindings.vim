function! tmcr#shared#configure_bindings()
  echo "Configure TmuxCreator commands"
  command! -nargs=+ TmuxCreatorLaunchSession call tmcr#open_session(<f-args>)
  command! -nargs=+ TmuxCreatorKillSession call tmcr#kill_session(<f-args>)
  command! -nargs=0 TmuxCreatorRegisteredSession call tmcr#list_session#registered()
  command! -nargs=0 TmuxCreatorListSession call tmcr#list_session#opened()
endfunction
