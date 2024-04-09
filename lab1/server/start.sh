docker stop redis-rust-container
docker rm redis-rust-container
docker build -t redis-rust .
docker run \
  --rm -d \
  --name redis-rust-container \
  -p 6379:6379 \
  -e RUST_LOG=debug \
  redis-rust
