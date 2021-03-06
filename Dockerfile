FROM ubuntu:latest


ENV TZ=America/Los_Angeles
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get clean \
&& apt-get update \
&& apt-get install sudo -y

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
&& sudo apt-get install zlib1g-dev -y \
&& sudo apt-get install net-tools -y \
&& sudo apt-get -y install openssl -y \
&& sudo apt-get install libssl-dev -y

RUN echo "alias ll='ls -l'" >> ~/.bashrc
RUN /bin/bash -c "history -a"
RUN /bin/bash -c "source ~/.bashrc"

RUN sudo -i
RUN curl https://packages.microsoft.com/keys/microsoft.asc | sudo apt-key add -
RUN curl https://packages.microsoft.com/config/ubuntu/18.04/prod.list > /etc/apt/sources.list.d/mssql-release.list
RUN apt-get update
RUN ACCEPT_EULA=Y apt-get install -y --allow-unauthenticated msodbcsql17
RUN ACCEPT_EULA=Y apt-get install -y --allow-unauthenticated mssql-tools
RUN echo 'export PATH="$PATH:/opt/mssql-tools/bin"' >> ~/.bash_profile
RUN echo 'export PATH="$PATH:/opt/mssql-tools/bin"' >> ~/.bashrc
RUN exit

ENV HOME /home/rust
ENV USER rust
ENV SHELL /bin/bash
WORKDIR /home/rust

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN echo "export PATH=~/.cargo/bin:$PATH" >> ~/.bashrc
RUN echo "export PS1='\u:\w$ '" >> ~/.bashrc
RUN /bin/bash -c "source ~/.bashrc"

WORKDIR /app

COPY . /app
ADD . /app

ENV OPENSSL_LIB_DIR "/usr/lib/x86_64-linux-gnu"
ENV OPENSSL_INCLUDE_DIR "/usr/include/openssl"

RUN ~/.cargo/bin/cargo install --path .

RUN sudo chmod +x geckodriver

CMD ["./geckodriver", "--port", "4444"]

CMD ["./target/release/mike_rust"]
