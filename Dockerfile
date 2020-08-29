FROM rust:1.31
FROM ubuntu:latest

FROM ubuntu:latest

ENV TZ=America/Los_Angeles
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get clean \
&& apt-get update \
&& apt-get install sudo -y

RUN useradd -m docker && echo "docker:docker" | chpasswd && adduser docker sudo

USER docker

RUN sudo apt-get install apt-transport-https -y \
&& sudo apt-get install unixodbc -y \
&& sudo apt-get install unixodbc-dev -y \
&& sudo apt-get install curl -y \
&& sudo apt-get install poppler-utils -y \
&& sudo apt-get install --reinstall build-essential -y \
&& sudo apt-get install file -y \
&& sudo apt-get install asciinema -y \
&& sudo apt-get install unzip -y \
&& sudo apt-get install vim -y \
&& sudo apt-get install nano -y \
&& sudo apt-get install git -y \
&& sudo apt-get install libssl-dev -y \
&& sudo apt-get install zlib1g-dev -y

RUN echo "Set disable_coredump false" >> /etc/sudo.conf

RUN sudo curl https://packages.microsoft.com/keys/microsoft.asc | apt-key add -
RUN sudo curl https://packages.microsoft.com/config/debian/9/prod.list > /etc/apt/sources.list.d/mssql-release.list
RUN sudo apt-get update
RUN sudo ACCEPT_EULA=Y apt-get install msodbcsql17
RUN sudo ACCEPT_EULA=Y apt-get install mssql-tools
RUN echo 'export PATH="$PATH:/opt/mssql-tools/bin"' >> ~/.bash_profile
RUN echo 'export PATH="$PATH:/opt/mssql-tools/bin"' >> ~/.bashrc


ENV HOME /home/rust
ENV USER rust
ENV SHELL /bin/bash
WORKDIR /home/rust

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN echo "export PATH=~/.cargo/bin:$PATH" >> ~/.bashrc
RUN echo "export PS1='\u:\w$ '" >> ~/.bashrc

COPY . /
ADD . /

RUN cargo install --path .

RUN sudo chmod +x geckodriver

CMD ["./geckodriver", "--port", "4444"]

CMD ["mike_rust"]
