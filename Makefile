GOCMD=go
GOFLAGS=-o trace -gcflags "all=-trimpath=$GOPATH"

all: clean darwin linux windows

clean:
	-rm -rf ./output

darwin:
	mkdir -p output
	GOOS=darwin GOARCH=amd64 $(GOCMD) build $(GOFLAGS) -o ./output/LatestSpigot-darwin

linux:
	mkdir -p output
	GOOS=linux GOARCH=386 $(GOCMD) build $(GOFLAGS) -o ./output/LatestSpigot-linux

windows:
	mkdir -p output
	GOOS=windows GOARCH=386 $(GOCMD) build $(GOFLAGS) -o ./output/LatestSpigot.exe