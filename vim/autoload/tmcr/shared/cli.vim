function! tmcr#shared#cli#call(arg)
  let cmd = call("printf", "tmcr " + a:arg)
  let res = system(cmd)

  if empty(res)
    throw "Failed to call TmuxCreator binary with arg: " . a:arg
  endif

  return res
endfunction

