#!/usr/bin/sh

PKGNAME=rshttpd
PREFIX=usr/local/bin
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

			cp "$SRCDIR/$PKGNAME" "$PKGNAME/$PREFIX/"
			cp "control" "$PKGNAME/DEBIAN/"

			dpkg-deb --build "$PKGNAME"
			;;
	esac
}

main $*
