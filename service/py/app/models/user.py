# from marshmallow.fields import Boolean
# from sqlalchemy.orm import defaultload
# from sqlalchemy.sql.base import ColumnSet

from sqlalchemy.sql.schema import ForeignKey
from sqlalchemy.orm import relation, relationship
from app.exts import db
from sqlalchemy import or_
from datetime import datetime
import hashlib
from flask import current_app
import uuid

from app.models.permission import Permission


# 用户
class User(db.Model):
    __tablename__ = "users"

    id = db.Column(db.Integer, primary_key=True)
    account_id = db.Column(db.String(100), default="")

    name = db.Column(db.String(100), unique=True)
    nick_name = db.Column(db.String(100), nullable=False, default="")
    phone = db.Column(db.String(30), default="")
    email = db.Column(db.String(30), default="")
    avatar = db.Column(db.Text, default="")

    introduction = db.Column(db.Text, default="")
    hashed_passwd = db.Column(db.String(200))

    is_active = db.Column(db.Boolean, nullable=False, default=False)
    is_deleted = db.Column(db.Boolean, nullable=False, default=False)
    is_logout = db.Column(db.Boolean, nullable=False, default=True)

    created_date = db.Column(db.DateTime, default=datetime.now())
    registration_date = db.Column(db.DateTime, default=datetime.now())
    active_date = db.Column(db.DateTime)
    login_date = db.Column(db.DateTime)

    role_id = db.Column(db.Integer, ForeignKey("roles.id"))
    role = relationship("Role")

    @classmethod
    def make_hashed_passwd(cls, passwd):
        salt = current_app.config.get("PASSWD_SALT")
        content = f'{salt}{passwd}'
        h = hashlib.md5(content.encode())
        return h.hexdigest()

    @classmethod
    def is_exists(cls, name, phone_num) -> bool:
        """
        check if user with name or phone_num
        has already exists.
        """
        condition = or_(User.name == name, User.phone_num == phone_num)
        query = User.query.filter(condition).exists()
        return db.session.query(query).scalar()

    @classmethod
    def get(cls, name_or_phone_num, passwd):
        passwd = cls.make_hashed_passwd(passwd)
        condition = or_(
            User.name == name_or_phone_num,
            User.phone == name_or_phone_num,
        )
        query = User.query.filter(condition).filter(
            User.hashed_passwd == passwd)
        for row in query:
            print(row)
        return query.one_or_none()

    @classmethod
    def load_user(cls, account_id):
        return User.query.filter(User.account_id == account_id).one_or_none()

    def get_id(self) -> str:
        return "{}".format(self.account_id)

    @property
    def is_authenticated(self):
        return True

    def has_permissions(self, *requirements):
        """ has_permissions(Permission.a, (Permission.b, Permission.c), Permission.d)
            Translates to:
                User has permission 'a' AND (role 'b' OR role 'c') AND role 'd'"""
        # Translates a list of role objects to a list of perms
        permission_code_set = self.role.permission_code_set()
        if Permission.All.value in permission_code_set:
            return True

        # has_role() accepts a list of requirements
        for requirement in requirements:
            if isinstance(requirement, (list, tuple)):
                # this is a tuple_of_perms requirement
                tuple_of_perms = requirement
                authorized = False
                for perm in tuple_of_perms:
                    if perm.value in permission_code_set:
                        authorized = True
                        break
                if not authorized:
                    return False  # tuple_of_perms requirement failed: return False
            else:
                perm = requirement
                if not perm.value in permission_code_set:
                    return False  # perm requirement failed: return False
        return True


class Role(db.Model):
    __tablename__ = "roles"

    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(50), unique=True)
    permission_code = db.Column(db.Text)

    def permission_code_set(self):
        return set(int(x) for x in self.permission_code.split(","))

    def set_permissions(self, permissions):
        perms = [str(x.value) for x in permissions]
        self.permission_code = ",".join(perms)


def create_admin(name, passwd):
    hashed_passwd = User.make_hashed_passwd(passwd)
    user = User(name=name, hashed_passwd=hashed_passwd)
    role = Role.query.filter(Role.name == "admin").one_or_none()
    if not role:
        role = Role(name="admin")
        role.set_permissions([Permission.All])
        db.session.add(role)

    user.is_active = True
    user.account_id = uuid.uuid4().hex
    user.role = role
    db.session.add(user)
    db.session.commit()
