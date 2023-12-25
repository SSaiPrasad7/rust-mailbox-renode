#!/bin/sh
set -x #echo on

# Override platform here
PLATFORM=core-v-mcu

BASEDIR=$(dirname "$0")
BIN=${BIN=$1}

PLATFORM_PATH=$BASEDIR/../platforms/core-v-mcu.repl

renode --console -e "set bin @$BIN; set platform_path @$PLATFORM_PATH; include @$BASEDIR/start.resc"
