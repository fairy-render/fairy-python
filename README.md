# Fairy python


## Devleopment

Needs maturin, the rust compiler, virtualenv and pnpm for build the frontend example

npm i -g pnpm
just build-frontend

pipx install maturin 
pipx install patchelf
pipx install uniffi-bindgen
python -m venv .venv

source .venv/activate.fish

maturin develop
python example