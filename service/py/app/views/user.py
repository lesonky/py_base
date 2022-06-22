from datetime import datetime
import click
import os
import uuid
from flask import current_app

from flask import Blueprint, request
from flask_jwt_extended import jwt_required
from flask_jwt_extended import create_access_token
from flask_jwt_extended import current_user

from app.exts import jwt, db
from app.core.result import ok, NotFound, Unauthorized
from app.models.user import User
from app.schemas.user import UserSchema, LoginUserSchema
from app.schemas.role import RoleSchema
from datetime import datetime
from app.models.user import create_admin

bp = Blueprint('user', __name__)


@jwt.user_identity_loader
def user_identity_lookup(user):
    return user.account_id


@jwt.user_lookup_loader
def user_lookup_callback(_jwt_header, jwt_data):
    identity = jwt_data["sub"]
    user = User.query.filter(User.account_id == identity,
                             User.is_active == True,
                             User.is_deleted == False).one_or_none()
    if not user:
        raise Unauthorized(f"id 为{identity}用户，不存在或已被冻结或已被删除，请联系管理员")

    user.active_date = datetime.now()
    db.session.commit()
    if user.is_logout:
        raise Unauthorized("用户已退出，请重新登录")
    return user


@bp.route('/user/login', methods=['POST'])
def login():
    args = LoginUserSchema().load(request.json)
    user = User.get(args['name'], args['password'])
    if not user:
        raise NotFound("用户不存在或者密码不匹配")
    user.login_at = datetime.now()
    user.is_logout = False
    db.session.commit()
    access_token = create_access_token(identity=user)
    resp = ok({"token": access_token})
    resp.set_cookie('token', access_token)
    return resp


@bp.route("/user/info", methods=["POST"])
@jwt_required()
def user_info():
    user_info = UserSchema().dump(current_user)
    return ok(user_info)


@bp.route('/user/logout', methods=['POST'])
@jwt_required()
def logout():
    current_user.is_logout = True
    db.session.commit()
    return ok()


@bp.route("/user/avatar", methods=['POST'])
@jwt_required()
def upload_avatar():
    print("the request files is", request.files)
    file = request.files['avatar']
    _, ext = os.path.splitext(file.filename)

    uid = uuid.uuid4().hex
    uuid_filename = "{}{}".format(uid, ext)
    avatar_path = os.path.join(current_app.config['AVATAR_PATH'],
                               uuid_filename)
    file.save(avatar_path)
    url = f"/api/user/avatar/{uuid_filename}"
    return ok({"avatar": url})


@bp.route('/user/avatar/<path:path>', methods=['GET'])
def avatar_image(path):
    from flask import current_app, send_from_directory
    return send_from_directory(current_app.config['AVATAR_PATH'], path)


@bp.cli.command('create_admin')
@click.argument('name')
@click.argument('passwd')
def cli_create_admin(name, passwd):
    create_admin(name, passwd)
