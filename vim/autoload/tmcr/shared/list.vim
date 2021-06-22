let s:cli = function("tmcr#shared#cli#call")
let s:picker = function("tmcr#shared#utils#picker")

function! tmcr#shared#list#registered()
    let s:registered_sessions = s:to_array(s:cli("-r"))
    call s:picker(s:registered_sessions, 'OPEN')
endfunction

function! tmcr#shared#list#opened()
    let s:opened_sessions = s:to_array(s:cli("-l"))
    call s:picker(s:opened_sessions, 'REGISTER')
endfunction

function! s:to_array(output)
  return split(a:output, ", ")
endfunction

