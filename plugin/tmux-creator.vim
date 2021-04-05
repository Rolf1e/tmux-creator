if !exists('s:tmuxCreatorJobId')
	let s:tmuxCreatorJobId= 0
endif

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
  command! -nargs=0 ListSession :call s:listSession()
endfunction

let s:ListSession = 'list'

function! s:listSession()
  call rpcrequest(s:tmuxCreatorJobId, s:ListSession)
endfunction

call s:connect()

