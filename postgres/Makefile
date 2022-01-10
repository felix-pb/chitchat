IMAGE := chitchat_postgres:latest
CONTAINER := chitchat-postgres-1
NETWORK := chitchat_net
VOLUME := chitchat_data

run:
	docker build --tag ${IMAGE} .

	docker run \
	--env POSTGRES_PASSWORD=password \
	--detach \
	--hostname postgres \
	--name ${CONTAINER} \
	--network ${NETWORK} \
	--publish 5432:5432 \
	--rm \
	--volume ${VOLUME}:/var/lib/postgresql/data \
	${IMAGE}

stop:
	docker stop ${CONTAINER}
