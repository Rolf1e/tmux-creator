if !exists('g:tmuxCreatorJobId')
	let s:tmuxCreatorJobId = 0
endif

let s:ListSession = 'list'
let s:RegisteredListSession = 'registered'

let s:tmux_creator_path_bin = '/media/rolfie/ssd2/projects/tmux-creator/target/release/neovim-plugin'

function! s:connect()
  let id = s:initRpc()
  
  if 0 == id
    echoerr "Tmux Creator: cannot start rpc process"
  elseif -1 == id
    echoerr "Tmux Creator: rpc process is not executable"
  else
    let s:tmuxCreatorJobId = id 
    echo "Tmux Creator: Start process with id " . s:tmuxCreatorJobId
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
  command! -nargs=0 RegisteredSession :call s:registeredListSession()
  command! -nargs=0 ListSession :call s:listSession()
endfunction


function! s:listSession()
  call rpcnotify(s:tmuxCreatorJobId, s:ListSession)
endfunction

function! s:registeredListSession()
  call rpcnotify(s:tmuxCreatorJobId, s:RegisteredListSession)
endfunction

call s:connect()

