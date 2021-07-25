FROM fedora:34
WORKDIR /usr/local/bin
COPY ./target/release/auth_microservice /usr/local/bin/auth_microservice
RUN dnf install curl -y && dnf clean all -y
STOPSIGNAL SIGINT
ENTRYPOINT ["auth_microservice"]
