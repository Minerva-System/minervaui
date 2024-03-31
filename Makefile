DOCKER_USERNAME ?= luksamuk
DOCKER_VERSION  := $(shell grep -m 1 "version" Cargo.toml | awk '{print $$3}' | tr -d '"')
DOCKER_NAME     := $(DOCKER_USERNAME)/minervaui
DOCKER_TAG      := $(DOCKER_NAME):$(DOCKER_VERSION)

.PHONY: docker run_docker

all:
	cargo build --release

run:
	cargo run

run_docker:
	docker run -it --rm $(DOCKER_NAME):latest

# docker:
# 	docker build . -f Dockerfile -t $(DOCKER_NAME):latest -t $(DOCKER_TAG)
# 	docker push $(DOCKER_TAG)
# 	docker push $(DOCKER_NAME):latest

docker:
	docker buildx build \
		--platform=linux/amd64,linux/arm64 \
		-f Dockerfile \
		-t $(DOCKER_NAME):latest \
		-t $(DOCKER_TAG) \
		--push \
		.
