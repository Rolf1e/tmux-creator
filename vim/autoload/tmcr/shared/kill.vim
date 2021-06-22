let s:cli = function("tmcr#shared#cli#call")

function! tmcr#shared#kill#kill_session(session)
    echo s:cli("-k " . a:session)
endfunction
