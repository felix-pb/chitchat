build:
	docker build -t chitchat .

run:
	docker run -dp 3000:3000 --name chitchat-app --rm chitchat

stop:
	docker stop chitchat-app
