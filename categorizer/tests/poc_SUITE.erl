-module(poc_SUITE).
-compile([export_all,nowarn_export_all]).

all() ->
    [initialize].

initialize(_Config) ->
    categorizer_app:start([],[]),
    categorizer_app:stop([]).