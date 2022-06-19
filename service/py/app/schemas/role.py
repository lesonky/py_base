from app.exts import ma
from app.models.user import Role
from app.models.permission import Permission
from marshmallow import Schema, fields, validate, post_dump


class RoleSchema(ma.SQLAlchemyAutoSchema):

    class Meta:
        model = Role

    @post_dump()
    def add_permission_code_names(self, data, many, **kwargs):
        code = data.pop('permission_code', "")
        names = [Permission(int(x)).name for x in code.split(',') if x]
        data['permissions'] = names
        return data


class ListRoleSchema(Schema):
    name = fields.String(required=False)
    pageNum = fields.Integer(missing=1, validate=validate.Range(min=1))
    pageSize = fields.Integer(missing=10,
                              validate=validate.Range(min=1, max=100))


class CreateRoleSchema(Schema):
    name = fields.String(required=True)
    permissions = fields.List(fields.String)


class EditRoleSchema(Schema):
    id = fields.Integer(required=True)
    name = fields.String()
    permissions = fields.List(fields.String)


class DeleteRoleSchema(Schema):
    id = fields.Integer(required=True)
