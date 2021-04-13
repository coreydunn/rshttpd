#!/usr/bin/sh

PKGNAME=rshttpd
PREFIX=usr/local/bin
SRCDIR=..

main()
{
	mkdir -p "$PKGNAME/DEBIAN"
	mkdir -p "$PKGNAME/$PREFIX"

	cp "$SRCDIR/$PKGNAME" "$PKGNAME/$PREFIX/"
	cp "control" "$PKGNAME/DEBIAN/"

	dpkg-deb --build "$PKGNAME"
}

main
