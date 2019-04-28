xargo_files:
	cp ../../../kernel/Cargo.* ./prime_xargo


clean_build: xargo_files
	docker build --no-cache -t jamesmintram/jimos_toolchain:1 .

build: xargo_files
	docker build -t jamesmintram/jimos_toolchain:1 .

docker-login:
	@echo "Env vars: DOCKER_USER DOCKER_PASS"
	docker login -u $(DOCKER_USER) -p $(DOCKER_PASS)

push: build docker-login
	docker push jamesmintram/jimos_toolchain:1
