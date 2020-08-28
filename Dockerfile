FROM rust:1.31
FROM ubuntu:latest

ENV TZ=America/Los_Angeles
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone
ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get clean \
&& apt-get update \
&& apt-get install sudo -y \
&& apt-get install apt-transport-https -y \
&& sudo apt-get install unixodbc -y \
&& sudo apt-get install unixodbc-dev -y \
&& sudo apt-get install curl -y \
&& sudo apt-get install poppler-utils -y \
&& sudo apt-get install --reinstall build-essential -y


RUN echo "Set disable_coredump false" >> /etc/sudo.conf

RUN sudo su
RUN sudo curl https://packages.microsoft.com/keys/microsoft.asc | apt-key add -
RUN sudo curl https://packages.microsoft.com/config/debian/9/prod.list > /etc/apt/sources.list.d/mssql-release.list
RUN sudo apt-get update
RUN sudo ACCEPT_EULA=Y apt-get install msodbcsql17
RUN sudo ACCEPT_EULA=Y apt-get install mssql-tools
RUN echo 'export PATH="$PATH:/opt/mssql-tools/bin"' >> ~/.bash_profile
RUN echo 'export PATH="$PATH:/opt/mssql-tools/bin"' >> ~/.bashrc


COPY . /
ADD . /

RUN sudo chmod +x geckodriver

CMD ["./geckodriver", "--port", "4444"]

CMD [ "cargo", "run" ]
