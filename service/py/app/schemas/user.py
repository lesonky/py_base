from marshmallow import Schema, fields, validate, EXCLUDE
from werkzeug.wrappers import request
from app.exts import ma
from app.models.user import User, Role
from app.core.schema import CamelCaseSchemaMixin, camelcase
from app.schemas.role import RoleSchema


class QueryUserSchema(CamelCaseSchemaMixin, Schema):
    """
    params of query user
    """
    phone = fields.Str()
    name = fields.Str()
    nick_name = fields.Str()
    email = fields.Str()
    role = fields.Str()
    include_deleted = fields.Boolean(missing=False, required=False)
    page_num = fields.Integer(missing=1, validate=validate.Range(min=1))
    page_size = fields.Integer(missing=10,
                               validate=validate.Range(min=1, max=100))


class UserUpertSchema(CamelCaseSchemaMixin, Schema):
    """
    fields for update or create user
    """
    # user uuid
    account_id = fields.String(required=False)
    name = fields.String(required=True)
    nick_name = fields.String()
    email = fields.String()
    avatar = fields.String()
    is_deleted = fields.Bool(required=False)
    is_active = fields.Bool(required=True)
    phone = fields.String()
    password = fields.String()
    introduction = fields.String()
    role = fields.Dict()

    class Meta:
        unknown = EXCLUDE


class ResetPasswordSchema(CamelCaseSchemaMixin, Schema):
    old_password = fields.String(required=True)
    new_password = fields.String(required=True)


class UserUpdateSelfSchema(CamelCaseSchemaMixin, Schema):
    nick_name = fields.String()
    email = fields.String()
    avatar = fields.String()
    phone = fields.String()
    introduction = fields.String()


class LoginUserSchema(Schema):
    """
    args for login user
    """
    name = fields.String(required=True)
    password = fields.String(required=True)


class UserSchema(CamelCaseSchemaMixin, ma.SQLAlchemyAutoSchema):

    class Meta:
        model = User
        include_fk = False
        exclude = ['hashed_passwd']

    role = fields.Nested(RoleSchema)
