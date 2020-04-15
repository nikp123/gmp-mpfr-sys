#!/bin/bash

# Copyright © 2017–2020 University of Malta

# Copying and distribution of this file, with or without modification,
# are permitted in any medium without royalty provided the copyright
# notice and this notice are preserved. This file is offered as-is,
# without any warranty.

# This script untars gmp, mpfr, mpc and tweaks them a little.

set -e

# Change the variables below before running

# library versions and tar locations
TARDIR="$HOME/Downloads"

GMPVER=6.2.0
GMPVERP="$GMPVER"
GMPTAR="$TARDIR/gmp-$GMPVER.tar.lz"
GMPPATCH="$TARDIR/gmp-$GMPVERP-allpatches"

MPFRVER=4.0.2
MPFRVERP="$MPFRVER-p7"
MPFRTAR="$TARDIR/mpfr-$MPFRVER.tar.xz"
MPFRPATCH="$TARDIR/mpfr-$MPFRVERP-allpatches"

MPCVER=1.1.0
MPCVERP="$MPCVER"
MPCTAR="$TARDIR/mpc-$MPCVER.tar.gz"
MPCPATCH="$TARDIR/mpc-$MPCVERP-allpatches"

CHANGELOG_CHARS=100000

function truncate {
	mv "$1" "$1.rm~"
	(
		if (($2 > 0)); then
			head -c $(($2 - 1))
			head -n 1
		fi
		if [ $(head -c 1 | wc -c) == 1 ]; then
			echo "... (truncated)"
		fi
	) < "$1.rm~" > "$1"
}

# GMP
# 1. Truncate ChangeLog
# 2. Remove doc/*.info*, doc/*.tex
# 3a. Remove demos section in configure
# 3b. Remove doc/Makefile, demos/{,*/}Makefile from ac_config_files in configure
# 4. Remove doc and demos from SUBDIRS in Makefile.in
# 5. In tests/misc/t-locale.c, add " && ! defined __ANDROID__" to "#if HAVE_NL_LANGINFO".
if [ -e gmp-*-c ]; then
	rm -r gmp-*-c
fi
tar xf "$GMPTAR"
mv gmp-$GMPVER gmp-$GMPVERP-c
cd gmp-$GMPVERP-c
if [ -f "$GMPPATCH" ]; then
    patch -N -Z -p1 < "$GMPPATCH" > /dev/null
fi
truncate ChangeLog $CHANGELOG_CHARS
rm doc/*.info* doc/*.tex
sed -i.rm~ -e '
/Configs for demos/,/Create config.m4/{
         /Create config.m4/!s/^/#gmp-mpfr-sys /
         s/\(#gmp-mpfr-sys\) $/\1/
}
/^ac_config_files=/{
        :repeat
        s/\( #gmp-mpfr-sys .*\) #gmp-mpfr-sys\(.*\)/\1\2/
        s,^\([^#]*\) \(\(doc\|demos[/a-z]*\)/Makefile\)\([^#]*\)\($\| #\),\1\4 #gmp-mpfr-sys \2\5,
        t repeat
}
' configure
sed -i.rm~ -e '
/^SUBDIRS = /{
	:repeat
        s/\( #gmp-mpfr-sys .*\) #gmp-mpfr-sys\(.*\)/\1\2/
        s,^\([^#]*\) \(doc\|demos\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
        t repeat
}
' Makefile.in
sed -i.rm~ -e 's/#if HAVE_NL_LANGINFO/& \&\& ! defined __ANDROID__/' tests/misc/t-locale.c
cd ..

# MPFR
# 1. Truncate ChangeLog
# 2. Remove doc/*.info*, doc/*.tex
# 3. Remove doc/Makefile, mpfr.pc from ac_config_files in configure
# 4a. Remove doc from SUBDIRS in Makefile.in
# 4b. Remove $(pkgconfig_DATA) from DATA in Makefile.in
# 5. Remove get_patches.c rule in src/Makefile.in
# 6. Generate src/get_patches.c
if [ -e mpfr-*-c ]; then
	rm -r mpfr-*-c
fi
tar xf "$MPFRTAR"
mv mpfr-$MPFRVER mpfr-$MPFRVERP-c
cd mpfr-$MPFRVERP-c
if [ -f "$MPFRPATCH" ]; then
    patch -N -Z -p1 < "$MPFRPATCH" > /dev/null
fi
truncate ChangeLog $CHANGELOG_CHARS
rm doc/*.info* doc/*.tex
sed -i.rm~ -e '
/^ac_config_files=/{
        :repeat
        s/\( #gmp-mpfr-sys .*\) #gmp-mpfr-sys\(.*\)/\1\2/
        s,^\([^#]*\) \(doc/Makefile\|mpfr.pc\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
        t repeat
}
' configure
sed -i.rm~ -e '
/^SUBDIRS = /s,^\([^#]*\) \(doc\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
/^DATA = /s,^\([^#]*\) \(\$(pkgconfig_DATA)\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
' Makefile.in
sed -i.rm~ '/get_patches.c:/,/^$/s/^\(.\)/#gmp-mpfr-sys \1/' src/Makefile.in
tools/get_patches.sh > src/get_patches.c
cd ..

# MPC
# 1. Make sure all files are user writeable
# 2. Truncate ChangeLog
# 3. Remove doc/*.info*, doc/*.tex
# 4. Remove doc/Makefile from ac_config_files in configure
# 5. Remove doc from SUBDIRS in Makefile.in
if [ -e mpc-*-c ]; then
	rm -r mpc-*-c
fi
tar xf "$MPCTAR"
mv mpc-$MPCVER mpc-$MPCVERP-c
cd mpc-$MPCVERP-c
if [ -f "$MPCPATCH" ]; then
    patch -N -Z -p1 < "$MPCPATCH" > /dev/null
fi
chmod -R u+w *
truncate ChangeLog $CHANGELOG_CHARS
rm doc/*.info* doc/*.tex
sed -i.rm~ '
/^ac_config_files=/s,^\([^#]*\) \(doc/Makefile\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
' configure
sed -i.rm~ '
/^SUBDIRS = /s,^\([^#]*\) \(doc\)\([^#]*\)\($\| #\),\1\3 #gmp-mpfr-sys \2\4,
' Makefile.in
cd ..

# Finally
# 1. Comment Makefile:...esac sections from all Makefile.in
# 2. Remove all .rm~ files left over by sed
for m in $(find *-c -name Makefile.in); do
    sed -i.rm~ '/Makefile:/,/esac/s/^/#gmp-mpfr-sys /' $m
done
find *-c -name \*.rm~ -delete

# Documentation
# 1. Build html documentation
# 2. Remove unnecessary node redirects
# 3. Remove anything outside <body>, including the <body> and </body> tags themselves
# 4. Remove blank lines (so that rustdoc's markdown interpreter sees as html)
# 5. Redirect directory links
# 6. Clear margins and padding for tables with class="menu", "index-cp", "index-fn"
# 8. Redirect links by prepending "constant.", replacing "-" and "_002d" by "_", and replacing "_002b" by "P"
if [ -e doc-c ]; then
    rm -r doc-c
fi
REMOVE_STRAY_BACKSLASHES='
s/mp\\_bits\\_per\\_limb/mp_bits_per_limb/g
s/GMP\\_NUMB\\_BITS/GMP_NUMB_BITS/g
s/\\log/log/g
s/\\exp/exp/g
s/\\pi/Pi/g
s/\\infty/Inf/g
'
mkdir doc-c{,/GMP,/MPFR,/MPC}
makeinfo gmp*/doc/gmp.texi --html --split=chapter --output=doc-c/GMP
makeinfo mpfr*/doc/mpfr.texi --html --split=chapter --output=doc-c/MPFR
makeinfo mpc*/doc/mpc.texi --html --split=chapter --output=doc-c/MPC
for f in doc-c/*/*.html; do
    if grep -q 'The node you are looking for is' "$f"; then
        rm "$f"
        continue
    fi
    sed -i.rm~ -e '0,/<body/d' "$f"
    sed -i.rm~ -e '/<\/body>/,$d' "$f"
    sed -i.rm~ -e '/^$/d' "$f"
    sed -i.rm~ -e "$REMOVE_STRAY_BACKSLASHES" "$f"
    sed -i.rm~ -e 's/..\/dir\/index.html\|dir.html#Top/..\/index.html/g' "$f"
    sed -i.rm~ -e '/<table class="menu"/,/<\/table>/s/<td\|<th/& style="padding: 0; border: 0;" /g' "$f"
    sed -i.rm~ -e '/<table class="index-/,/<\/table>/s/<td\|<th/& style="padding: 1px; border: 0;" /g' "$f"
    sed -i.rm~ -e 's/<table class="\(menu\|index-[cpfn]*\)"/& style="margin: 0; width: auto; padding: 0; border: 0;"/' "$f"
    sed -i.rm~ -e ': repeat; s/"\([A-Z][A-Za-z0-9_-]*\.html\)/"constant.\1/; t repeat' "$f"
    sed -i.rm~ -e ': repeat; s/\("constant\.[A-Za-z0-9_]*\)-\([A-Za-z0-9_-]*\.html\)/\1_\2/; t repeat' "$f"
    sed -i.rm~ -e ': repeat; s/\("constant\.[A-Za-z0-9_]*\)_002b\([A-Za-z0-9_]*\.html\)/\1P\2/; t repeat' "$f"
    sed -i.rm~ -e ': repeat; s/\("constant\.[A-Za-z0-9_]*\)_002d\([A-Za-z0-9_]*\.html\)/\1_\2/; t repeat' "$f"
done
find doc-c -name \*.rm~ -delete
