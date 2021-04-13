#!/usr/bin/sh

PKGNAME=rshttpd
PREFIX=usr/local/bin
CONFDIR=etc
SRCDIR=..

main()
{
	case "$1" in
		clean)
			rm -rf "$PKGNAME" "$PKGNAME.deb"
			exit
			;;

		*)
			mkdir -p "$PKGNAME/DEBIAN"
			mkdir -p "$PKGNAME/$PREFIX"
			mkdir -p "$PKGNAME/$CONFDIR"

			cp "$SRCDIR/$PKGNAME" "$PKGNAME/$PREFIX/"
			cp "control" "$PKGNAME/DEBIAN/"
			cp "rshttpd.conf" "$PKGNAME/$CONFDIR/"

			dpkg-deb --build "$PKGNAME"
			;;
	esac
}

main $*
