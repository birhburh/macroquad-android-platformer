#!/bin/sh
#
# Copyright (C) 2018 daisuke-t
#
# Make android icon(ic_launcher.png) on Mac OS.
#
# Arguments:
#   $1 - source image
#
# Note:
#   Source image is PNG 24 format.

set -e

# check args.
if [ $# -ne 1 ]; then
	echo "invalid argument."
	echo $#
	exit
fi

SRC_FILE=$1
OUT_DIR=android_res


# prepare build folder.
mkdir -p $OUT_DIR/mipmap-mdpi
mkdir -p $OUT_DIR/mipmap-hdpi
mkdir -p $OUT_DIR/mipmap-xhdpi
mkdir -p $OUT_DIR/mipmap-xxhdpi
mkdir -p $OUT_DIR/mipmap-xxxhdpi


# make icon.
convert -limit time 2000 -scale 48x48 ${SRC_FILE} $OUT_DIR/mipmap-mdpi/ic_launcher.png
convert -limit time 2000 -scale 72x72 ${SRC_FILE} $OUT_DIR/mipmap-hdpi/ic_launcher.png
convert -limit time 2000 -scale 96x96 ${SRC_FILE} $OUT_DIR/mipmap-xhdpi/ic_launcher.png
convert -limit time 2000 -scale 144x144 ${SRC_FILE} $OUT_DIR/mipmap-xxhdpi/ic_launcher.png
convert -limit time 2000 -scale 192x192 ${SRC_FILE} $OUT_DIR/mipmap-xxxhdpi/ic_launcher.png
