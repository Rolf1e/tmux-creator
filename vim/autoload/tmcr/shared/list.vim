let s:cli = function("tmcr#shared#cli#call")

function! tmcr#list_session#registered()
    return s:format_output(s:cli("-r"))
endfunction

function! tmcr#list_session#opened()
    return s:format_output(s:cli("-l"))
endfunction

function! s:format_output(output)
  return split(output, ", ")
endfunction

