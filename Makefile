.PHONY: start deploy

all: start

start:
	cargo web start

deploy:
	cargo web deploy --release
	rm -rf docs/*
	cp -r target/deploy/* docs
