%#!/usr/bin/env escript
-mode(compile).

main([X]) ->
  io:format("~p~n", [fib(list_to_integer(X))]).

fib(I) ->
  fib2(0, 1, I).

fib2(_, I2, X) when X =< 0 ->
  I2;
fib2(I1, I2, X) ->
  fib2(I2, I1+I2, X-1).
