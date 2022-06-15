import uuid
from datetime import datetime
from app.exts import db
from flask_login import login_required
from flask import Blueprint, jsonify, request
from sqlalchemy import exc

from flask_jwt_extended import jwt_required

from app.core.decorator import roles_required
from app.core.result import ok, NotFound
from app.models.user import User, UserRoles, Role
from app.exts import jwt

from marshmallow import Schema, fields

from app.schemas.user import RoleSchema, UserSchema, QueryUserSchema, UserUpertSchema
from app.core.result import BadParam

bp = Blueprint('user_admin', __name__)


@bp.route('/user/list', methods=['GET'])
@jwt_required()
def query_users():
    args = QueryUserSchema().load(request.args)
    query = User.query.filter()
    page = query.order_by(User.id.desc()).paginate(page=args['pageNum'],
                                                   per_page=args['pageSize'])
    users_schema = UserSchema(many=True)
    items = users_schema.dump(page.items)

    # inject user roles
    user_ids = [x['id'] for x in items]
    q = UserRoles.query.filter(UserRoles.user_id.in_(user_ids))
    m = {}
    role_map = Role.role_map()
    for row in q:
        m[row.user_id] = m.get(row.user_id, [])
        m[row.user_id].append({
            "id": row.role_id,
            'name': role_map.get(row.role_id)
        })
    for item in items:
        item['roles'] = m.get(item['id'])
    return ok({'total': page.total, 'items': items})


@bp.route("/user/roles", methods=["GET"])
@jwt_required()
@roles_required("admin")
def option_roles():
    roles = Role.query.filter()
    items = RoleSchema(many=True).dump(roles)
    return ok({"items": items})


def update_user(args):
    args.pop('name')
    account_id = args['account_id']
    passwd = args.pop('password', None)

    user = User.query.filter(User.account_id == account_id).one_or_none()
    if not user:
        raise NotFound(f"No user found with id {account_id}")
    if passwd:
        user.hashed_passwd = User.make_hashed_passwd(passwd)

    for key, value in args.items():
        setattr(user, key, value)

    roles = args.pop("roles", [])
    role_ids = [x['id'] for x in roles]
    has_ids = [
        x.id for x in UserRoles.query.filter(UserRoles.user_id == user.id)
    ]
    to_delete_ids = [x for x in has_ids if x not in role_ids]
    to_create_ids = [x for x in role_ids if x not in has_ids]

    if to_delete_ids:
        query = UserRoles.query.filter(UserRoles.user_id == user.id,
                                       UserRoles.role_id.in_(to_delete_ids))
        query.delete(synchronize_session=False)
    for role_id in to_create_ids:
        user_role = UserRoles(user_id=user.id, role_id=role_id)
        db.session.add(user_role)
    db.session.commit()
    return user


def create_user(args):
    roles = args.pop("roles", [])
    passwd = args.pop('password', None)
    if not passwd:
        raise BadParam("Password is required for create new user")

    user = User(**args)
    user.hashed_passwd = User.make_hashed_passwd(passwd)
    user.account_id = uuid.uuid4().hex
    db.session.add(user)
    db.session.commit()

    for role in roles:
        user_role = UserRoles(role_id=role['id'], user_id=user.id)
        db.session.add(user_role)
    db.session.commit()
    return user


@bp.route("/user/upsert", methods=['POST'])
@jwt_required()
@roles_required('admin')
def user_upsert():
    print(request.json)
    args = UserUpertSchema().load(request.json)
    user = None
    account_id = args.get("account_id", None)
    try:
        if account_id:
            user = update_user(args)
        else:
            user = create_user(args)
    except exc.IntegrityError as e:
        if "UNIQUE constraint failed" in str(e):
            raise BadParam(f"用户名 {args['name']} 已存在")
    user_info = UserSchema().dump(user)
    user_info['roles'] = RoleSchema(many=True).dump(user.query_roles())
    return ok(user_info)
