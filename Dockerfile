FROM clux/muslrust
RUN mkdir /source
WORKDIR /source
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./korrecte-autogen/ /source/korrecte-autogen/
COPY ./korrecte-cli/ /source/korrecte-cli/
COPY ./korrecte-dev/ /source/korrecte-dev/
COPY ./korrecte-lib/ /source/korrecte-lib/
COPY ./korrecte-web/ /source/korrecte-web/
COPY ./kubeclient/ /source/kubeclient/
RUN cargo build --release --bin korrecte-web
RUN strip ./target/x86_64-unknown-linux-musl/release/korrecte-web
RUN cargo build --release --bin korrecte-cli
RUN strip ./target/x86_64-unknown-linux-musl/release/korrecte-cli
COPY ./korrecte.toml .

FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=0 /source/target/x86_64-unknown-linux-musl/release/korrecte-web /
COPY --from=0 /source/target/x86_64-unknown-linux-musl/release/korrecte-cli /
COPY --from=0 /source/korrecte.toml /