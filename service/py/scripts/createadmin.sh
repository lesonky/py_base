#!/bin/bash

export FLASK_ENV=development 
export FLASK_DEBUG=True
export FLASK_APP=app/run.py 

flask user create_admin admin admin