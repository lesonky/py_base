from flask import Blueprint, request
from sqlalchemy import exc

from flask_jwt_extended import jwt_required
from app.core.decorator import permission_required
from app.core import result
from app.schemas.role import ListRoleSchema, RoleSchema, CreateRoleSchema
from app.schemas.role import DeleteRoleSchema, EditRoleSchema
from app.models.permission import Permission
from app.models.user import Role, User
from app.exts import db

bp = Blueprint('role', __name__)


@bp.route("/role/list", methods=["GET"])
@jwt_required()
@permission_required(Permission.ListRole)
def list_role():
    m = Role
    args = ListRoleSchema().load(request.args)
    query = m.query.filter()
    page = query.order_by(m.id.desc()).paginate(page=args['pageNum'],
                                                per_page=args['pageSize'])
    schemas = RoleSchema(many=True)
    items = schemas.dump(page.items)
    return result.ok({"items": items, "total": page.total})


@bp.route("/role/create", methods=["POST"])
@jwt_required()
@permission_required(Permission.CreateRole)
def create_role():
    args = CreateRoleSchema().load(request.json)
    name = args['name']
    permissons = Permission.from_str_list(args['permissions'])
    role = Role(name=name)
    role.set_permissions(permissons)
    try:
        db.session.add(role)
        db.session.commit()
    except exc.IntegrityError as e:
        raise result.BadParam(f"role with name {role.name} already exist")
    return result.ok(RoleSchema().dump(role))


@bp.route("/role/update", methods=["POST"])
@jwt_required()
@permission_required(Permission.EditRole)
def update_role():
    args = EditRoleSchema().load(request.json)
    role = Role.query.filter(Role.id == args['id']).one_or_none()
    if not role:
        raise result.BadParam(f"role with id {role.id} not found")
    perms = Permission.from_str_list(args['permissions'])
    role.set_permissions(perms)
    db.session.commit()
    role_json = RoleSchema().dump(role)
    return result.ok(role_json)


@bp.route("/role/delete", methods=["POST"])
@jwt_required()
@permission_required(Permission.DeleteRole)
def delete_role():
    args = DeleteRoleSchema().load(request.json)
    role = Role.query.filter(Role.id == args['id']).one_or_none()
    if not role:
        raise result.BadParam(f"role with id {role.id} not found")
    role_users = User.query.filter(User.role_id == role.id)
    user_names = [x.name for x in role_users]
    if user_names:
        raise result.BadParam(
            f"user [{user_names}] have role, please switch role of them first")
    db.session.delete(role)
    db.session.commit()
    return result.ok()


@bp.route("/permission/list", methods=["GET"])
@jwt_required()
@permission_required(Permission.ListPermission)
def list_permission():
    return result.ok({"permissions": Permission.all_permission_name()})
