IMAGE := chitchat_frontend:latest
CONTAINER := chitchat-frontend-1
NETWORK := chitchat_net

run:
	docker build --tag ${IMAGE} .

	docker run \
	--detach \
	--name ${CONTAINER} \
	--network ${NETWORK} \
	--publish 8080:8080 \
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
	npm run lint

watch:
	docker build --file Dockerfile.watch --tag ${IMAGE} .

	docker run \
	--detach \
	--name ${CONTAINER} \
	--network ${NETWORK} \
	--publish 8080:8080 \
	--rm \
	--volume $(shell pwd)/src:/app/src \
	${IMAGE} \
	npm run serve
