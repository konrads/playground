%#!/usr/bin/env escript
-mode(compile).

main([X]) ->
  io:format("~p~n", [fact(list_to_integer(X))]).

fact(I) ->
  fact2(I, 1).

fact2(0, Acc) ->
  Acc;
fact2(I, Acc) ->
  fact2(I-1, I*Acc).
