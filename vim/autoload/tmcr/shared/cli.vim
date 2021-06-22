function! tmcr#shared#cli#call(arg)
  let res = system("tmcr " . a:arg)

  if empty(res)
    throw "Failed to call TmuxCreator binary with arg: " . a:arg
  endif

  return res
endfunction

function! tmcr#shared#cli#call_void(arg)
  return system("tmcr " . a:arg)
endfunction

