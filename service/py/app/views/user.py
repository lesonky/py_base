from flask import Blueprint, jsonify, request
from flask_migrate import current
from hobbit_core.utils import secure_filename
from flask_login import login_user, login_required, logout_user, current_user
from app.exts import db, login_manager
from marshmallow import Schema, fields, validate
from app.utils.result import ok, err_badparam
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


@bp.route('/user/register', methods=['POST'])
def register():
    """
    register user with phone num and verify code
    """
    pass


@bp.route('/user/logout', methods=['GET'])
@login_required
def logout():
    logout_user()
    return ok()


@bp.route('/user/profile', methods=['GET'])
@login_required
def profile():
    """
    return current user basic info
    and menu permissions
    """
    data = UserSchema().dump(current_user)
    org = Orgnization.query.filter(
        Orgnization.id == current_user.org_id).one_or_none()
    if org:
        data['org'] = OrgnizationSchema().dump(org)
    title = JobTitle.query.filter(
        JobTitle.id == current_user.title_id).one_or_none()
    if title:
        data['title'] = title.name
    department = Department.query.filter(
        Department.id == current_user.department_id).one_or_none()
    if department:
        data['department'] = department.name
    data['perms'] = []
    data['roles'] = RoleSchema(many=True).dump(current_user.roles)
    return ok(data)


@bp.route("/user/upload/avatar", methods=['POST'])
def upload_avatar():
    file = request.files.get('file')
    filename = secure_filename(file.filename)
    _, ext = os.path.splitext(filename)

    uid = uuid.uuid4().hex
    uuid_filename = "{}{}".format(uid, ext)
    avatar_path = os.path.join(current_app.config['AVATAR_PATH'],
                               uuid_filename)
    file.save(avatar_path)
    url = f"/files/avatar/{uuid_filename}"
    return ok({"url": url, 'avatar_file': uuid_filename})


@bp.route('/files/avatar/<path:path>', methods=['GET'])
def avatar_image(path):
    return send_from_directory(current_app.config['AVATAR_PATH'], path)


@bp.route('/menu/list', methods=['GET'])
def user_menu():
    pass
