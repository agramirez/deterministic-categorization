-module(llama_SUITE).
-include_lib("stdlib/include/assert.hrl").
-moduledoc """

""".
-compile([export_all,nowarn_export_all]).

all() ->
    [{group,basic}].

groups() ->
    [{basic,[parallel],[
        can_start
        % ,can_get_new_embedding
        % ,can_get_cached_embedding
        ,can_cache_embedding
    ]}].

can_start(_Config) ->
    {ok,Actual}=start_generic(),
    ?assert(is_pid(Actual)).

can_cache_embedding(_Config) ->
    {ok,Server}=start_generic(test_cache_embedding),
    llama_srv:cache_embedding(Server,<<"hello world">>),
    ok.


start_generic() ->
    start_generic(test1).

start_generic(ServerName) ->
    ServerEndpoint="server:11434",
    Json="application/json",
    ExpectedBody=#{},
    Response=#{},
    ExpectedEmbedding=[],
    RequestFun=fun (post,{MyUrl,[{"Content-Type",MyJson}],MyOtherJson,MyBody}) ->
        ?assertEqual(MyJson, Json),
        ?assertEqual(MyOtherJson, Json),
        ?assertEqual(MyBody, json:encode(ExpectedBody)),
        ?assertEqual(MyUrl, ServerEndpoint),
        {ok,{{"HTTP 1.0",200,"OK"},[{"Content-Type",Json}],json:encode(Response)}}
    end,
    EmbeddingsModel="nomic",
    Params=#{},
    GetEmbeddingFun=fun (MyHash) ->
        case MyHash of
            _ ->
                {error,notfound}
        end
    end,
    SetEmbeddingFun=fun (MyHash,MyText,MyEmbedding) ->
        ?assertEqual(MyHash, erlang:phash2(string:trim(string:lowercase(MyText)))),
        ?assertEqual(MyEmbedding,ExpectedEmbedding),
        ok
    end,
    Repo=#{embeddings=>#{get=>GetEmbeddingFun,set=>SetEmbeddingFun}},
    llama_srv:start_link(ServerName, RequestFun, ServerEndpoint, EmbeddingsModel, Params, Repo).
