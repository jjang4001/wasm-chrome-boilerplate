FROM rust:1.30-slim

RUN apt-get -yqq update;		\
	apt-get -yqq install curl

# wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# cargo-generate
RUN apt-get -yqq install pkg-config libssl-dev git;	\
	cargo install cargo-generate;
ENV USER root

# nodejs
RUN apt-get -yqq install gnupg;									\
	curl -sL https://deb.nodesource.com/setup_10.x | bash -;	\
	apt-get install -yqq nodejs

# yarn
RUN apt-get -yqq remove cmdtest yarn;																	\
	apt-get -y autoremove;																				\
	curl -sS https://dl.yarnpkg.com/debian/pubkey.gpg | apt-key add -;									\
	echo "deb https://dl.yarnpkg.com/debian/ stable main" | tee /etc/apt/sources.list.d/yarn.list;		\
	apt-get update;																						\
	apt-get install -yqq yarn;																			\
	yarn install

WORKDIR /usr/src

# 8080 webpack-dev-server. still have to bind at runtime
EXPOSE 8080 5000
