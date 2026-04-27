FROM devcontainer:general

# install erlang
RUN sudo apt install -y erlang rebar3

# install python dependencies
RUN sudo apt install -y python3 python3-pip python3-venv
RUN curl -LsSf https://astral.sh/uv/install.sh | sudo sh
RUN python3 -m venv ./.venv && ./.venv/bin/python3 -m pip install ipykernel psycopg2-binary