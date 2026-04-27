-module(llama_srv).
-behaviour(gen_server).
-moduledoc """

""".
-include_lib("kernel/include/logger.hrl").

-export([start_link/6,stop/1]).
-export([cache_embedding/2]).
-export([init/1,terminate/2,handle_cast/2,handle_call/3]).

start_link(Name,RequestFun,ServerEndpoint,EmbeddingsModel,Params,Repo) ->
    gen_server:start_link({local,Name},?MODULE,[RequestFun,ServerEndpoint,EmbeddingsModel,Params,Repo],[]).

stop(Name) ->
    gen_server:stop(Name).

-doc """
If an embedding of the specified text does not exist in cache, it creates it and caches it.
""".
-spec cache_embedding(Server::pid(),Text::bitstring()) -> ok.
cache_embedding(Server,Text) when is_pid(Server), is_binary(Text) ->
    ok=gen_server:cast(Server, {embed,Text}),
    ok.

init([RequestFun,ServerEndpoint,EmbeddingsModel,Params,Repo]) ->
    {ok,#{server=>#{url=>ServerEndpoint,request=>RequestFun},model=>#{embeddings=>EmbeddingsModel},params=>Params,repo=>Repo}}.

terminate(_Reason,_State) ->
    ok.

handle_call({embed,Text},_From,State=#{embeddings:=Embeddings,repo:=Repo,server:=#{url:=Server,request:=RequestFun},model:=#{embeddings:=Model}}) ->
    CleanText=string:trim(string:lowercase(Text)),
    Hash=erlang:phash2(CleanText),
    case lists:member(Hash,Embeddings) of
        true ->
            #{embeddings:=#{get:=GetEmbedding}}=Repo,
            {ok,{Hash,CleanText,Embedding}}=GetEmbedding(Hash),
            {reply,{ok,Embedding},State};
        false ->
            {ok,Embedding}=get_embedding(RequestFun,Server,Model,CleanText),
            #{embeddings:=#{set:=SetEmbeddings}}=Repo,
            ok=SetEmbeddings(Hash,CleanText,Embedding),
            {reply,{ok,Embedding},State#{embeddings=>[Hash|Embeddings]}}
    end.

handle_cast({embed,Text},State=#{embeddings:=Embeddings,repo:=Repo,server:=#{url:=Server,request:=RequestFun},model:=#{embeddings:=Model}}) ->
    CleanText=string:trim(string:lowercase(Text)),
    Hash=erlang:phash2(CleanText),
    case lists:member(Hash,Embeddings) of
        true ->
            {noreply,State};
        false ->
            {ok,Embedding}=get_embedding(RequestFun,Server,Model,CleanText),
            #{embeddings:=#{set:=SetEmbeddings}}=Repo,
            ok=SetEmbeddings(Hash,CleanText,Embedding),
            {noreply,State#{embeddings=>[Hash|Embeddings]}}
    end.

get_embedding(RequestFun,Server,Model,Text) ->
    ?LOG_DEBUG("requesting embedding for ~p using model ~p and server ~p",[Text,Model,Server]),
    Url= <<Server/binary,"/v1/embedding">>,
    Json="application/json",
    Body=#{~B"stream"=>false,~B"model"=>Model,~B"input"=>Text},
    Req={Url,[],Json,json:encode(Body)},
    HttpOps=[{timeout, 30000},{connect_timeout,6000}],
    Ops=[{sync, true},{body_format, binary}],
    case RequestFun(post,Req,HttpOps,Ops) of
         {{_HttpVersion, _StatusCode, _StatusText}, _HttpHeader, HttpBodyResult} ->
            ?LOG_DEBUG("received response: ~p",[HttpBodyResult]),
            {ok,json:decode(HttpBodyResult)};
        Unk ->
            ?LOG_DEBUG("received unkown response: ~p",[Unk]),
            {ok,[]}
    end.