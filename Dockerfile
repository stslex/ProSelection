FROM rust:buster AS build
COPY . /code
WORKDIR /code
RUN cargo build --release

FROM debian:buster-slim
EXPOSE 8000:8000
RUN apt update
RUN apt upgrade
RUN apt install libpq5 -y
COPY --from=build /code/target/release/pro_selection /pro_selection
CMD [ "/pro_selection" ]
