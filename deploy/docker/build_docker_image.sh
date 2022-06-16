#/bin/bash
export SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
export PROJECT_DIR=$(dirname $(dirname $SCRIPT_DIR))

source $SCRIPT_DIR/$DEPLOY_ENV

set -x
tag=$(cat $SCRIPT_DIR/build_version_hash)
echo "the tag is $tag"


DOCKER_IMAGE_WEB_API="webace_base_api"
DOCKER_IMAGE_FE="webace_base_fe"

docker build -f $PROJECT_DIR/fe/Dockerfile -t $DOCKER_IMAGE_FE:$tag $PROJECT_DIR/fe
docker build -f $PROJECT_DIR/service/py/Dockerfile -t $DOCKER_IMAGE_WEB_API:$tag $PROJECT_DIR/service/py/

