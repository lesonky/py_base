from flask import Blueprint, jsonify, request
from flask_migrate import current
from hobbit_core.utils import secure_filename
from flask_login import login_user, login_required, logout_user, current_user
from app.exts import db, login_manager
from marshmallow import Schema, fields, validate
from app.utils.result import ok
from flask import current_app
from flask import send_from_directory
import os
import uuid
from datetime import datetime

bp = Blueprint('user', __name__)


@login_manager.user_loader
def load_user(user_id):
    pass


@bp.route('/user/login', methods=['POST'])
def login():
    return ok()


@bp.route('/user/logout', methods=['GET'])
@login_required
def logout():
    logout_user()
    return ok()
