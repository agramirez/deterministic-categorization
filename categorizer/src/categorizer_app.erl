-module(categorizer_app).
-behaviour(application).
-moduledoc """

""".
-export([start/2, stop/1]).

start(_StartType, _StartArgs) ->
    categorizer_sup:start_link().

stop(_State) ->
    ok.
