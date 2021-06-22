let s:cli = function("tmcr#shared#cli#call")

function! tmcr#open_session(session)
    return s:cli("-a " + a:session)
endfunction
