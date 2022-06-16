
#!/bin/bash
export SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
export PROJECT_DIR=$(dirname $(dirname $SCRIPT_DIR))

source $SCRIPT_DIR/$DEPLOY_ENV
$SCRIPT_DIR/sync.h

ssh -t "$SERVER_HOST" "cd $SERVER_DST_PATH && DEPLOY_ENV=$DEPLOY_ENV ./deploy/docker/build_docker_image.sh"
ssh -t "$SERVER_HOST" "cd $SERVER_DST_PATH && DEPLOY_ENV=$DEPLOY_ENV ./deploy/docker/stop.sh web_api"
ssh -t "$SERVER_HOST" "cd $SERVER_DST_PATH && DEPLOY_ENV=$DEPLOY_ENV ./deploy/docker/stop.sh fe"
ssh -t "$SERVER_HOST" "cd $SERVER_DST_PATH && DEPLOY_ENV=$DEPLOY_ENV ./deploy/docker/run.sh up -d"
