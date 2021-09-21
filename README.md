# Build
`docker build -t myapp .`

# Run (without ENV)
` docker run --name myapp-container-name -p 8000:8000 --rm -it myapp`

# Run (with ENV)
` docker run --name myapp-container-name -e RUST_DOCKER_NAME=rust-docker-name-test -p 8000:8000 --rm -it myapp`

# Stop 
`sudo docker stop myapp-container-name`

# Remove
`sudo docker rm myapp-container-name`

# Run with docker-compose
`sudo docker-compose up -d`

# Stop with docker-compose
`sudo docker-compose down`