all: src/rshttpd
src/rshttpd: src/main.rs src/connection.rs
	rustc $< -o $@
	@strip $@
	@[ -e rshttpd ] || ln -s $@ ./rshttpd
pkg: rshttpd.1.gz pkg/rshttpd.deb
pkg/rshttpd.deb: src/rshttpd
	cd pkg && ./mkpkg.sh
%.gz: %
	[ -f $@ ] || gzip -k $^
clean:
	$(RM) src/rshttpd
	[ -d pkg ] && cd pkg && ./mkpkg.sh clean
	$(RM) rshttpd.1.gz
