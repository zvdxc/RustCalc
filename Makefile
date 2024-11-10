
all: build


run:
	cargo run

build:
	cargo build -r
	cp -v target/release/untitled rustcalc

clean:
	rm -vr target
	rm -v rustcalc

install:
	cargo install --path .

install-alt:
	cargo build
	sudo cp -v target/release/untitled /usr/local/bin/rustcalc

uninstall:
	cargo uninstall

uninstall-alt:
	sudo rm -v /usr/local/bin/rustcalc