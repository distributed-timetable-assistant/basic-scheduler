FROM scratch
COPY target/x86_64-unknown-linux-musl/release/basic-scheduler /basic-scheduler
CMD ["/basic-scheduler"]
EXPOSE 3000