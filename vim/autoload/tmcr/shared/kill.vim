let s:cli = function("tmcr#shared#cli#call")

function! tmcr#kill_session(session)
    return s:cli("-k " + a:session)
endfunction
