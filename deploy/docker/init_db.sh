#!/bin/bash
set -x
export SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
export PROJECT_DIR=$(dirname $(dirname $SCRIPT_DIR))

source $SCRIPT_DIR/$DEPLOY_ENV

sql=$(cat <<-EOF
create database $MYSQL_DB_NAME;
create user '$MYSQL_DB_USER'@'%'  IDENTIFIED WITH mysql_native_password by '$MYSQL_DB_PASS';
grant all PRIVILEGES ON $MYSQL_DB_NAME.* to '$MYSQL_DB_USER'@'%';
EOF
)

echo $sql|mysql -u root --password=$MYSQL_ROOT_PASSWORD -h 127.0.0.1 -P $MYSQL_HOST_PORT
