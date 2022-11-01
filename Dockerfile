FROM alpine as build-environment
WORKDIR /opt
RUN apk add clang lld curl build-base linux-headers git pkgconfig openssl-dev\
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh \
    && chmod +x ./rustup.sh \
    && ./rustup.sh -y

RUN [[ "$TARGETARCH" = "arm64" ]] && echo "export CFLAGS=-mno-outline-atomics" >> $HOME/.profile || true

WORKDIR /opt/conduit
COPY . .
RUN source $HOME/.profile && cargo build --release \
    && strip /opt/conduit/target/release/conduit

FROM alpine as conduit-client
ENV GLIBC_KEY=https://alpine-pkgs.sgerrand.com/sgerrand.rsa.pub
ENV GLIBC_KEY_FILE=/etc/apk/keys/sgerrand.rsa.pub
ENV GLIBC_RELEASE=https://github.com/sgerrand/alpine-pkg-glibc/releases/download/2.35-r0/glibc-2.35-r0.apk

RUN apk add linux-headers gcompat git
RUN wget -q -O ${GLIBC_KEY_FILE} ${GLIBC_KEY} \
    && wget -O glibc.apk ${GLIBC_RELEASE} \
    && apk add glibc.apk --force
COPY --from=build-environment /opt/conduit/target/release/conduit /usr/local/bin/conduit

RUN adduser -Du 1000 conduit
ENTRYPOINT ["/bin/sh", "-c"]