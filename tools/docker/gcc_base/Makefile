
build:
	docker build -t jamesmintram/jimos_toolchain:1 .

docker-login:
	docker login -u $(DOCKER_USER) -p $(DOCKER_PASS)

push: build docker-login
	docker push jamesmintram/jimos_toolchain:1
