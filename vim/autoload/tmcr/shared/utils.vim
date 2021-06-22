let s:open_session = function("tmcr#shared#open#open_session")
let s:kill_session = function("tmcr#shared#kill#kill_session")

function! tmcr#shared#utils#picker(sessions, type)
  if exists('*fzf#run')
    call s:fzf_picker(a:sessions, a:type)
  else
    call s:native_picker(a:sessions, a:type)
  endif
endfunction

function! s:native_picker(sessions, type)
  echo a:type . 'ED sessions: ' . a:sessions 
  # TODO
endfunction


function! s:fzf_picker(sessions, type)
    if a:type ==? "OPEN"
      call s:fzf_picker_(a:sessions, s:open_session)
    elseif a:type ==? "REGISTER"
      call s:fzf_picker_(a:sessions, s:kill_session)
    else
      throw "Bad action type " . a:type
    endif
endfunction

function! s:fzf_picker_(sessions, func)
  call fzf#run({
    \'source': a:sessions,
    \'sink': a:func,
    \'window': {'width': 0.9, 'height': 0.6},
    \ })
endfunction


