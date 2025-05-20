#!/bin/bash
# Script to setup an ubuntu-22.04 container for building.
# e.g.:
# podman run \
#     --network=host \
#     --volume=$PWD:/src/z2edit \
#     -it \
#     --rm \
#     ubuntu:22.04 \
#     /src/z2edit/util/ubuntu-22.04-setup.sh

apt update
DEBIAN_FRONTEND=noninteractive apt install -y \
	build-essential \
	curl \
	gettext \
	git \
	libgtk-3-dev \
	libgtk2.0-dev \
	libsdl2-dev \
	pkg-config \
	python-is-python3 \
	python3-dev \
	python3-pkg-resources \
	python3-venv \
	ssh \
	vim

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source "$HOME/.cargo/env"
python -mvenv venv
source venv/bin/activate
cd /src/z2edit
pip install -U -r python-requirements.txt 
exec bash -i
