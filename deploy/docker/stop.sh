#!/bin/bash
export SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
export PROJECT_DIR=$(dirname $SCRIPT_DIR)

source $SCRIPT_DIR/$DEPLOY_ENV

export CM_DIR=$SCRIPT_DIR/cm/$CM_ENV
export YML_DIR=$SCRIPT_DIR/services

docker-compose \
  -p $PROJECT_NAME \
  -f $YML_DIR/fe.yml\
  -f $YML_DIR/web.yml\
  stop $@

docker-compose \
  -p $PROJECT_NAME\
  -f $YML_DIR/fe.yml\
  -f $YML_DIR/web.yml\
  rm -f -v $@
