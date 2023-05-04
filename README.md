# Build

※ protobuf 必要かもしれないので起こられたら `brew install protobuf` とかよしなに。
`cargo build`

# Run

`cargo run`

# Check

- `curl -vvv -X POST http://localhost:8080/echo -H 'Content-Type: application/json' -d '{ "message": "hoge" }'`
- `grpcurl -plaintext -proto ./proto/echo.proto -d '{ "message": "piyo" }' localhost:8080 examples.Echo.UnaryEcho`
