FROM debian:trixie-20260316

RUN apt update && apt upgrade -y

# Include lld linker to improve build times either by using environment variable
# RUSTFLAGS="-C link-arg=-fuse-ld=lld" or with Cargo's configuration file (i.e see .cargo/config.toml).
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
   && apt-get -y install clang lld \
   && apt-get autoremove -y && apt-get clean -y

# install basic tools
RUN apt install -y curl
RUN apt install -y wget
RUN apt install -y git
RUN apt install -y libclang-dev clang

# install rocq
RUN apt-get install -y linux-libc-dev
RUN apt-get install -y m4
RUN apt-get install -y opam
RUN apt-get install -y time
RUN apt-get install -y zip
RUN apt-get install -y zsh
RUN apt-get install -y libgmp3-dev
RUN DEBIAN_FRONTEND=noninteractive apt-get install -y pkg-config

ARG USERNAME=cis5000
ARG USER_UID=1000
ARG USER_GID=$USER_UID

RUN apt-get update -y

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME --shell /bin/zsh \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

COPY hack.sh /tmp/hack.sh
RUN chmod +x /tmp/hack.sh
RUN /tmp/hack.sh
COPY .zshrc /home/$USERNAME/
USER $USERNAME

RUN opam init -y --disable-sandboxing --compiler=5.3.0
RUN opam switch 5.3.0
RUN opam install -y num
RUN opam repo add -y coq-released https://coq.inria.fr/opam/released
RUN opam pin add -y coq 9.0.0
RUN opam install -y coq-simple-io
RUN opam install -y vscoq-language-server
RUN opam update -y
RUN opam upgrade -y
RUN eval `opam config env`

# install latex
RUN sudo apt install -y texlive-full 

# install graphviz
RUN sudo apt install -y graphviz

# install plantuml
ARG PLANTUML_VERSION=1.2026.2
ENV PLANTUML_URL=https://github.com/plantuml/plantuml/releases/download/v${PLANTUML_VERSION}/plantuml-${PLANTUML_VERSION}.jar
ENV PLANTUML_DIR=/usr/share/plantuml
ENV PLANTUML_JAR=$PLANTUML_DIR/plantuml.jar
RUN sudo apt install -y plantuml
RUN sudo wget ${PLANTUML_URL}  -O /usr/share/plantuml/plantuml.jar

# install locales
RUN sudo apt-get install -y locales && locale-gen en_US.UTF-8

# additional setup for plantuml in latex
RUN sudo tlmgr init-usertree

# install rust
RUN sudo apt install -y build-essential rustup

RUN /workspaces/deterministic-normalization/.venv/bin/python -m pip install ipykernel -U --force-reinstall
RUN /workspaces/deterministic-normalization/.venv/bin/python -m pip install psycopg2-binary