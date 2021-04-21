if !exists('s:tmux_creator_job_id')
	let s:tmux_creator_job_id = 0
endif

" let s:tmux_creator_path_bin = '/media/rolfie/ssd2/projects/tmux-creator/target/release/neovim-plugin'
let s:tmux_creator_path_bin = './plugin/tmux-creator/target/release/neovim-plugin'
let s:ListSession = 'list'
let s:RegisteredListSession = 'registered'
let s:LaunchSession = 'launch'
let s:KillSession = 'kill'

function! s:connect()
  let id = s:initRpc()
  
  if 0 == id
    echoerr "Tmux Creator: cannot start rpc process"
  elseif -1 == id
    echoerr "Tmux Creator: rpc process is not executable"
  else
    let s:tmux_creator_job_id = id 
    call s:configureCommands() 
  endif
endfunction

function! s:initRpc()
  if s:tmux_creator_job_id == 0
    let jobid = jobstart([s:tmux_creator_path_bin], { 'rpc': v:true })
    return jobid
  else
    return s:tmux_creator_job_id 
  endif
endfunction

function! s:configureCommands()
  command! -nargs=+ TmuxCreatorLaunchSession :call s:rpcMessageOneParameter(s:LaunchSession, <f-args>)
  command! -nargs=+ TmuxCreatorKillSession :call s:rpcMessageOneParameter(s:KillSession, <f-args>)
  command! -nargs=0 TmuxCreatorRegisteredSession :call s:rpcMessage(s:RegisteredListSession)
  command! -nargs=0 TmuxCreatorListSession :call s:rpcMessage(s:ListSession)
endfunction

function! s:rpcMessage(...) 
  let s:message = get(a:, 1, 0)
  echo rpcrequest(s:tmux_creator_job_id, s:message)
endfunction

function! s:rpcMessageOneParameter(...) 
  let s:message = get(a:, 1, 0)
  let s:session_name = get(a:, 2, 0)
  echo rpcrequest(s:tmux_creator_job_id, s:message, s:session_name)
endfunction

call s:connect()

