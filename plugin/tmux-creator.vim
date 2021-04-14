if !exists('g:tmuxCreatorJobId')
	let s:tmuxCreatorJobId = 0
endif

let s:ListSession = 'list'
let s:RegisteredListSession = 'registered'
let s:LaunchSession = 'launch'
let s:KillSession = 'kill'

let s:tmux_creator_path_bin = '/media/rolfie/ssd2/projects/tmux-creator/target/release/neovim-plugin'

function! s:connect()
  let id = s:initRpc()
  
  if 0 == id
    echoerr "Tmux Creator: cannot start rpc process"
  elseif -1 == id
    echoerr "Tmux Creator: rpc process is not executable"
  else
    let s:tmuxCreatorJobId = id 
    call s:configureCommands() 
  endif
endfunction

function! s:initRpc()
  if s:tmuxCreatorJobId == 0
    let jobid = jobstart([s:tmux_creator_path_bin], { 'rpc': v:true })
    return jobid
  else
    return s:tmuxCreatorJobId 
  endif
endfunction

function! s:configureCommands()
  command! -nargs=+ LaunchSession :call s:rpcMessageOneParameter(s:LaunchSession, <f-args>)
  command! -nargs=+ KillSession :call s:rpcMessageOneParameter(s:KillSession, <f-args>)
  command! -nargs=0 RegisteredSession :call s:rpcMessage(s:RegisteredListSession)
  command! -nargs=0 ListSession :call s:rpcMessage(s:ListSession)
endfunction

function! s:rpcMessage(...) 
  let s:message = get(a:, 1, 0)
  call rpcnotify(s:tmuxCreatorJobId, s:message)
endfunction

function! s:rpcMessageOneParameter(...) 
  let s:message = get(a:, 1, 0)
  let s:session_name = get(a:, 2, 0)
  call rpcnotify(s:tmuxCreatorJobId, s:message, s:session_name)
endfunction

call s:connect()

