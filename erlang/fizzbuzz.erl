%#!/usr/bin/env escript
-mode(compile).

main([X]) ->
  io:format("~p~n", [fizzbuzz(list_to_integer(X))]).

lazy_take(X, Fun) -> lazy_take(1, X, Fun).

lazy_take(X, X, Fun) -> [Fun(X)];
lazy_take(X, Y, Fun) -> [Fun(X) | lazy_take(X+1, Y, Fun)].

fizz(X) when X rem 3 == 0 -> "fizz";
fizz(_) -> "".

buzz(X) when X rem 5 == 0 -> "buzz";
buzz(_) -> "".

fizzbuzz(X) ->
  Fizzes = lazy_take(X, fun fizz/1),
  Buzzes = lazy_take(X, fun buzz/1),
  [F++B || {F, B} <- lists:zip(Fizzes, Buzzes)].
