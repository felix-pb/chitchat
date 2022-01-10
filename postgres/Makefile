IMAGE := chitchat-postgres:latest
CONTAINER := chitchat-postgres-1

run:
	docker build --tag ${IMAGE} .

	docker run \
	--env POSTGRES_PASSWORD=password \
	--detach \
	--hostname postgres \
	--name ${CONTAINER} \
	--network chitchat-net \
	--publish 5432:5432 \
	--rm \
	--volume chitchat-data:/var/lib/postgresql/data \
	${IMAGE}

stop:
	docker stop ${CONTAINER}
