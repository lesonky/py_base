from marshmallow import Schema, fields, validate
from werkzeug.wrappers import request
from app.exts import ma
from app.models.user import User, Role
from app.core.schema import CamelCaseSchemaMixin, camelcase


class QueryUserSchema(Schema):
    """
    params of query user
    """
    phone = fields.Str()
    username = fields.Str()
    email = fields.Str()
    pageNum = fields.Integer(missing=1, validate=validate.Range(min=1))
    pageSize = fields.Integer(missing=10,
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
    roles = fields.List(fields.Dict())


class RegisterUserSchema(Schema):
    """
    fields when register user
    """
    name = fields.String(required=True)
    phone_num = fields.String(required=True)
    passwd = fields.String(required=True,
                           validate=validate.Length(min=8, max=16))
    confirmpasswd = fields.String(required=True,
                                  validate=validate.Length(min=8, max=16))
    gender = fields.Str(required=True,
                        validate=validate.OneOf(["male", "female"]))
    profile = fields.Str(required=True, validate=validate.Length(max=100))
    username = fields.String(required=True)
    age = fields.Integer()
    org_id = fields.Integer(required=True)
    department_id = fields.Integer(required=True)
    title_id = fields.Integer(required=True)
    verify_code = fields.Str(required=True)
    avatar_url = fields.Str()


class LoginUserSchema(Schema):
    """
    args for login user
    """
    name = fields.String(required=True)
    password = fields.String(required=True)


class RoleSchema(ma.SQLAlchemyAutoSchema):

    class Meta:
        model = Role


class UserSchema(CamelCaseSchemaMixin, ma.SQLAlchemyAutoSchema):

    class Meta:
        model = User
        include_fk = True
        exclude = ['hashed_passwd']
