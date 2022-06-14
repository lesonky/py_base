from marshmallow import Schema, fields, validate
from app.exts import ma
from app.models.user import Orgnization, User, Role


class QueryUserSchema(Schema):
    """
    params of query user
    """
    phone_num = fields.Str()
    name = fields.Str()
    title = fields.Str()
    department = fields.Str()
    is_active = fields.Boolean()
    page = fields.Integer(missing=1, validate=validate.Range(min=1))
    page_size = fields.Integer(missing=10,
                               validate=validate.Range(min=1, max=100))


class ActiveUserSchema(Schema):
    """
    args for active/deactive user
    """
    id = fields.Integer(required=True)
    is_active = fields.Boolean(required=True)


class OrgnizationSchema(ma.SQLAlchemyAutoSchema):

    class Meta:
        model = Orgnization


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
    name_or_phone_num = fields.String(required=True)
    passwd = fields.String(required=True)


class SendVerifyCodeSchema(Schema):
    """
    args for send verify code
    """
    phone_num = fields.String()


class ResetUserPasswdSchema(Schema):
    """
    args for reset user passwd
    """
    phone_num = fields.String(required=True)
    verify_code = fields.String(required=True)
    new_passwd = fields.String(required=True,
                               validate=validate.Length(min=8, max=16))


class UserSchema(ma.SQLAlchemyAutoSchema):

    class Meta:
        model = User
        include_fk = True
        exclude = ['hashed_passwd']

    org = fields.Nested(OrgnizationSchema)


class RoleSchema(ma.SQLAlchemyAutoSchema):

    class Meta:
        model = Role
