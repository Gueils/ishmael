FROM rust

RUN apt-get update && apt-get install -y libssl1.0-dev gdb

RUN export DOCKER_BUCKET=get.docker.com \
                         && export DOCKER_VERSION=1.12.3 \
                         && export DOCKER_SHA256=626601deb41d9706ac98da23f673af6c0d4631c4d194a677a9a1a07d7219fa0f \
                         && curl -fSL "https://${DOCKER_BUCKET}/builds/Linux/x86_64/docker-${DOCKER_VERSION}.tgz" -o docker.tgz \
                         && echo "${DOCKER_SHA256} *docker.tgz" | sha256sum -c - \
                         && tar -xzvf docker.tgz \
                         && mv docker/* /usr/local/bin/ \
                                        && rmdir docker \
                                        && rm docker.tgz

RUN mkdir -p /code

ADD . /code

WORKDIR /code

ENV GITHUB_API_KEY fake
RUN cargo install
