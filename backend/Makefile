IMAGE := chitchat-backend:latest
CONTAINER := chitchat-backend-1

run:
	docker build --tag ${IMAGE} .

	docker run \
	--detach \
	--init \
	--name ${CONTAINER} \
	--network chitchat-net \
	--publish 3000:3000 \
	--rm \
	${IMAGE}

stop:
	docker stop ${CONTAINER}

lint:
	docker build --file Dockerfile.lint --tag ${IMAGE} .

	docker run \
	--name ${CONTAINER} \
	--rm \
	--volume $(shell pwd)/src:/app/src \
	${IMAGE} \
	bash -c "cargo fmt && cargo clippy"

test:
	docker build --file Dockerfile.test --tag ${IMAGE} .

	docker run \
	--name ${CONTAINER} \
	--network chitchat-net \
	--rm \
	${IMAGE} \
	cargo test -- --test-threads=1

watch:
	docker build --file Dockerfile.watch --tag ${IMAGE} .

	docker run \
	--detach \
	--init \
	--name ${CONTAINER} \
	--network chitchat-net \
	--publish 3000:3000 \
	--rm \
	--volume $(shell pwd)/src:/app/src \
	${IMAGE} \
	cargo watch -x run
