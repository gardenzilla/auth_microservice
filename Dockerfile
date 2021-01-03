ARG package_name=auth_microservice
FROM debian:buster-slim
WORKDIR /usr/local/bin
COPY ./target/release/auth_microservice /usr/local/bin/auth_microservice
RUN apt-get update && apt-get install -y
RUN apt-get install curl -y
STOPSIGNAL SIGINT
ENTRYPOINT ["auth_microservice"]