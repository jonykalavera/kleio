FROM rustlang/rust:nightly

WORKDIR /app/kleio

COPY . .

RUN cargo install --path .

CMD ["kleio"]