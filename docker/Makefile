#
# docker factory
#
.PHONY = build run clean

build: clean
	@git clone https://github.com/Software-Knife-and-Tool/.git
	@docker build -f Dockerfile.build .
	@docker build -t ubuntu-dyad -f Dockerfile.ubuntu .
	@rm -rf dyad

run:
	docker run -it ubuntu-dyad /opt/dyad/bin/dyad

clean:
	@rm -rf dyad
