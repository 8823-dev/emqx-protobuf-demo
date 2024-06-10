# Memo

1. protoc がなかった
`$ brew install protobuf`

1. EMQX コンテナ起動
`$ docker run -d --name emqx -p 1883:1883 -p 8083:8083 -p 8084:8084 -p 8883:8883 -p 18083:18083 emqx/emqx:latest`

1. cargo run

# Docs

[Get Started with EMQX](https://docs.emqx.com/en/emqx/latest/getting-started/getting-started.html#next-steps)

[Crate rumqttc](https://docs.rs/rumqttc/latest/rumqttc/)

[Protocol Buffers (Encoding)](https://protobuf.dev/programming-guides/encoding/)