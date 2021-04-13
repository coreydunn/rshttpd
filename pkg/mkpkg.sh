#!/usr/bin/sh

PKGNAME=rshttpd
PREFIX=usr/local/bin
CONFDIR=etc
SRCDIR=..
INITDIR=etc/systemd/system

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
			mkdir -p "$PKGNAME/$INITDIR"

			cp "$SRCDIR/$PKGNAME" "$PKGNAME/$PREFIX/"
			cp "rshttpd.sh" "$PKGNAME/$PREFIX/"
			cp "control" "$PKGNAME/DEBIAN/"
			cp "rshttpd.conf" "$PKGNAME/$CONFDIR/"
			cp "rshttpd.service" "$PKGNAME/$INITDIR/"

			dpkg-deb --build "$PKGNAME"
			;;
	esac
}
main $*
