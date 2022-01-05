IMAGE := chitchat:latest
CONTAINER := chitchat-app

build:
	docker build -t ${IMAGE} .

run:
	docker run -dp 3000:3000 --init --name ${CONTAINER} --rm ${IMAGE}

stop:
	docker stop ${CONTAINER}
