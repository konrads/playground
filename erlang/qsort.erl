#!/usr/bin/env escript
main(Args) ->
  io:format("~s~n", [string:join(qsort(Args), ", ")]).

qsort([]) -> [];
qsort([X]) -> [X];
qsort([H | T]) ->
  Less = [X || X <- T, X < H],
  More = [X || X <- T, X >= H],
  qsort(Less) ++ [H] ++ qsort(More).
