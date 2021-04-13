all: src/rshttpd
src/rshttpd: src/main.rs src/connection.rs
	rustc $< -o $@
	@strip $@
	@[ -e rshttpd ] || ln -s $@ ./rshttpd
pkg: pkg/rshttpd.deb
pkg/rshttpd.deb: src/rshttpd
	cd pkg && ./mkpkg.sh
clean:
	$(RM) src/rshttpd
	[ -d pkg ] && cd pkg && ./mkpkg.sh clean
