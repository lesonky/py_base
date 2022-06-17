import uuid
from app.exts import db
from flask import Blueprint, request
from sqlalchemy import exc

from flask_jwt_extended import jwt_required
from flask_jwt_extended import current_user

from app.models.user import User, Role
from app.exts import jwt
from app.core.decorator import permission_required
from app.core.result import ok, NotFound, BadParam

from marshmallow import Schema, fields

from app.schemas.user import UserSchema, QueryUserSchema, UserUpdateSelfSchema, UserUpertSchema, ResetPasswordSchema
from app.models.permission import Permission

bp = Blueprint('user_admin', __name__)


@bp.route('/user/list', methods=['GET'])
def query_users():
    args = QueryUserSchema().load(request.args)
    query = User.query.filter()
    if args.get('phone', None):
        query = query.filter(User.phone.ilike(f'%{args["phone"]}%'))
    if args.get('name', None):
        query = query.filter(User.name.ilike(f'%{args["name"]}%'))
    if args.get("email", None):
        query = query.filter(User.email.ilike(f'%{args["email"]}%'))
    if args.get("nick_name", None):
        query = query.filter(User.nick_name.ilike(f'%{args["nick_name"]}%'))
    if not args.get("include_Deleted", None):
        query = query.filter(User.is_deleted == False)
    if args.get("role", None):
        role = Role.query.filter(Role.name == args['role']).one_or_none()
        if not role:
            raise BadParam(f"no role found with name {args['role']}")
        query = query.filter(User.role_id == role.id)

    page = query.order_by(User.id.desc()).paginate(page=args['page_num'],
                                                   per_page=args['page_size'],
                                                   error_out=False)
    users_schema = UserSchema(many=True)
    items = users_schema.dump(page.items)

    return ok({'total': page.total, 'items': items})


def get_role(args):
    role = args.pop("role", {})
    role_name = role.pop('name', None)
    if not role_name:
        raise BadParam("role: {name: xxx} required")
    role = Role.query.filter(Role.name == role_name).one_or_none()
    if not role:
        raise BadParam(f"no role found with name {role_name}")
    return role


def update_user(args):
    args.pop('name')
    role = get_role(args)
    account_id = args['account_id']
    passwd = args.pop('password', None)

    user = User.query.filter(User.account_id == account_id).one_or_none()
    if not user:
        raise NotFound(f"No user found with id {account_id}")
    if passwd:
        user.hashed_passwd = User.make_hashed_passwd(passwd)

    for key, value in args.items():
        setattr(user, key, value)
    user.role_id = role.id
    db.session.commit()
    return user


def create_user(args):
    passwd = args.pop('password', None)
    if not passwd:
        raise BadParam("Password is required for create new user")
    role = get_role(args)

    user = User(**args)
    user.hashed_passwd = User.make_hashed_passwd(passwd)
    user.account_id = uuid.uuid4().hex
    user.role_id = role.id

    db.session.add(user)
    db.session.commit()

    return user


@bp.route("/user/upsert", methods=['POST'])
@jwt_required()
@permission_required(Permission.EditUser)
def user_upsert():
    args = UserUpertSchema().load(request.json)
    account_id = args.get("account_id", None)
    user = None
    try:
        if account_id:
            user = update_user(args)
        else:
            user = create_user(args)
    except exc.IntegrityError as e:
        raise BadParam(f"用户名 {args['name']} 已存在")
    user_info = UserSchema().dump(user)
    return ok(user_info)


@bp.route("/user/self/update", methods=["POST"])
@jwt_required()
def user_update_self():
    args = UserUpdateSelfSchema().load(request.json)
    user = current_user
    for key, value in args.items():
        setattr(user, key, value)
    db.session.commit()
    user_info = UserSchema().dump(user)
    return ok(user_info)


@bp.route("/user/self/reset_password", methods=["POST"])
@jwt_required()
def user_reset_password():
    args = ResetPasswordSchema().load(request.json)
    user = current_user
    old_hashed_passwd = User.make_hashed_passwd(args['old_password'])
    if old_hashed_passwd != user.hashed_passwd:
        raise BadParam("旧密码不正确，请重新输入")
    user.hashed_passwd = User.make_hashed_passwd(args['new_password'])
    db.session.commit()
    user_info = UserSchema().dump(user)
    return ok(user_info)
