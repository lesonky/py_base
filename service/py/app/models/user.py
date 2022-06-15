# from marshmallow.fields import Boolean
# from sqlalchemy.orm import defaultload
# from sqlalchemy.sql.base import ColumnSet

from sqlalchemy.sql.schema import ForeignKey
from sqlalchemy.orm import relationship
from app.exts import db
from sqlalchemy import or_
from datetime import datetime
import hashlib
from flask import current_app
import uuid


# 用户
class User(db.Model):
    __tablename__ = "users"

    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(100), unique=True)
    nick_name = db.Column(db.String(100), nullable=False, default="")
    phone = db.Column(db.String(30), default="")
    email = db.Column(db.String(30), default="")

    is_active = db.Column(db.Boolean, nullable=False, default=False)
    is_deleted = db.Column(db.Boolean, nullable=False, default=False)
    is_logout = db.Column(db.Boolean, nullable=False, default=True)

    account_id = db.Column(db.String(100), default="")
    avatar = db.Column(db.Text, default="")
    introduction = db.Column(db.Text, default="")

    created_date = db.Column(db.DateTime, default=datetime.now())
    registration_date = db.Column(db.DateTime, default=datetime.now())
    active_date = db.Column(db.DateTime)
    login_date = db.Column(db.DateTime)
    hashed_passwd = db.Column(db.String(200))

    roles = relationship("UserRoles")

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

    def has_roles(self, *requirements):
        """ Return True if the user has all of the specified roles. Return False otherwise.

            has_roles() accepts a list of requirements:
                has_role(requirement1, requirement2, requirement3).

            Each requirement is either a role_name, or a tuple_of_role_names.
                role_name example:   'manager'
                tuple_of_role_names: ('funny', 'witty', 'hilarious')
            A role_name-requirement is accepted when the user has this role.
            A tuple_of_role_names-requirement is accepted when the user has ONE of these roles.
            has_roles() returns true if ALL of the requirements have been accepted.

            For example:
                has_roles('a', ('b', 'c'), d)
            Translates to:
                User has role 'a' AND (role 'b' OR role 'c') AND role 'd'"""

        # Translates a list of role objects to a list of role_names
        role_names = self.get_role_names()

        # has_role() accepts a list of requirements
        for requirement in requirements:
            if isinstance(requirement, (list, tuple)):
                # this is a tuple_of_role_names requirement
                tuple_of_role_names = requirement
                authorized = False
                for role_name in tuple_of_role_names:
                    if role_name in role_names:
                        authorized = True
                        break
                if not authorized:
                    return False  # tuple_of_role_names requirement failed: return False
            else:
                role_name = requirement
                if not role_name in role_names:
                    return False  # role_name requirement failed: return False
        return True

    def get_role_names(self):
        q = self.query_roles()
        return [row.name for row in q]

    def query_roles(self):
        q = db.session.query(Role).join(UserRoles,
                                        UserRoles.role_id == Role.id)
        q = q.filter(UserRoles.user_id == self.id)
        return q


class UserRoles(db.Model):
    __tablename__ = "user_roles"
    id = db.Column(db.Integer, primary_key=True)
    user_id = db.Column(db.Integer, ForeignKey("users.id", ondelete="CASCADE"))
    role_id = db.Column(db.Integer, ForeignKey("roles.id", ondelete="CASCADE"))


class Role(db.Model):
    __tablename__ = "roles"

    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(50), unique=True)
    permission_code = db.Column(db.Text)

    @classmethod
    def role_map(cls):
        query = cls.query.filter()
        return {row.id: row.name for row in query}


def create_admin(name, passwd):
    hashed_passwd = User.make_hashed_passwd(passwd)
    user = User(name=name, hashed_passwd=hashed_passwd)
    user.is_active = True
    user.account_id = uuid.uuid4().hex
    role = Role.query.filter(Role.name == "admin").one_or_none()
    if not role:
        role = Role(name="admin")
        db.session.add(role)
    db.session.add(user)
    db.session.commit()

    user_role = UserRoles(role_id=role.id, user_id=user.id)
    db.session.add(user_role)
    db.session.commit()


def init_roles():
    for name in ["admin", "user"]:
        role = Role.query.filter(Role.name == name).one_or_none()
        if not role:
            role = Role(name=name)
            db.session.add(role)
            print("add role", role, role.name)
    db.session.commit()
