let s:cli = function("tmcr#shared#cli#call_void")

function! tmcr#shared#open#open_session(session)
    echo s:cli("-a " . a:session)
endfunction
