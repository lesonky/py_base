#!/bin/bash
#export SERVER_DST=xieyu@192.168.0.14:/home/xieyu/webace_base
export SERVER_HOST=xieyu@192.168.0.14
export SERVER_DST_PATH=/home/xieyu/webace_base

export PROJECT_NAME=webace_base
export CM_ENV=test

# mysql 相关配置
export MYSQL_ROOT_PASSWORD=webace
export MYSQL_HOST_PORT=3308
export DATA_DIR=/opt/data/webace_base

# docker images
export WEB_API_TAG=$(cat $SCRIPT_DIR/build_version_hash)
export FE_TAG=$(cat $SCRIPT_DIR/build_version_hash)
export DOCKER_IMAGE_WEB_API="webace_base_api"
export DOCKER_IMAGE_FE="webace_base_fe"

# PORT
# 前端对外接口
export FE_HOST_PORT=9523
export WEB_API_HOST_PORT=9524


MYSQL_DB_NAME=webace_base
MYSQL_DB_USER=webace_base
MYSQL_DB_PASS=webace_base_123

