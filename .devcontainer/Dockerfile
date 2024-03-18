# Rust の公式 Devcontainer イメージをベースに使用
FROM mcr.microsoft.com/devcontainers/rust:1

# パッケージマネージャのアップデートとNode.jsのインストール
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends nodejs

# Devcontainer のメタデータを指定
LABEL cpm.distribution="Ubuntu"