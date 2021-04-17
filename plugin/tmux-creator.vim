if !exists('s:tmux_creator_job_id')
	let s:tmux_creator_job_id = 0
endif

let s:tmux_creator_path_bin = '/media/rolfie/ssd2/projects/tmux-creator/target/release/neovim-plugin'
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
  command! -nargs=+ LaunchSession :call s:rpcMessageOneParameter(s:LaunchSession, <f-args>)
  command! -nargs=+ KillSession :call s:rpcMessageOneParameter(s:KillSession, <f-args>)
  command! -nargs=0 RegisteredSession :call s:popupWindow(s:RegisteredListSession)
  command! -nargs=0 ListSession :call s:popupWindow(s:ListSession)
endfunction

function! s:rpcMessage(...) 
  let s:message = get(a:, 1, 0)
  return rpcrequest(s:tmux_creator_job_id, s:message)
endfunction

function! s:rpcMessageOneParameter(...) 
  let s:message = get(a:, 1, 0)
  let s:session_name = get(a:, 2, 0)
  call rpcrequest(s:tmux_creator_job_id, s:message, s:session_name)
endfunction

function! s:popupWindow(...) abort
  let s:message = get(a:, 1, 0)
  let width = 50
  let height = 50

  let window_buffer = nvim_create_buf(v:false, v:true)
  let ui = nvim_list_uis()[0]

  let response = s:rpcMessage(s:message)
  call nvim_buf_set_lines(window_buffer, 0, -1, v:true, response)

  let closingKeys = ['<Esc>', 'q']
  for closingKey in closingKeys
      call nvim_buf_set_keymap( window_buffer, 'n', closingKey, ':close<CR>', {'silent': v:true, 'nowait': v:true, 'noremap': v:true})
  endfor

  if !exists('g:tmux_creator_border_chars')
    let g:tmux_creator_border_chars = 'single'
  endif

  let opts = { 
        \ 'relative': 'editor',
        \ 'width': width, 
        \ 'height': height,
        \ 'col': (ui.width/2) - (width/2),
        \ 'row': (ui.height/2) - (height/2),
        \ 'anchor': 'NW',
        \ 'style': 'minimal',
        \ 'border': g:tmux_creator_border_chars,
        \}

  let window = nvim_open_win(window_buffer, 1, opts)

endfunction

call s:connect()

