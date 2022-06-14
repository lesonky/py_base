from enum import unique
from marshmallow.fields import Integer
from sqlalchemy.orm import relationship
from sqlalchemy.sql.schema import ForeignKey
from app.exts import db
from sqlalchemy import or_
from datetime import datetime
import hashlib
from flask import current_app


# 用户
class User(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(200), unique=True)
    username = db.Column(db.String(100))
    phone_num = db.Column(db.String(50), unique=True)
    avatar_url = db.Column(db.String(200), nullable=True)
    hashed_passwd = db.Column(db.String(256))
    gender = db.Column(db.String(20))
    age = db.Column(db.Integer)
    profile = db.Column(db.Text, nullable=True)
    org_id = db.Column(db.Integer, ForeignKey('orgnization.id'))
    title_id = db.Column(db.Integer, nullable=True)
    department_id = db.Column(db.Integer, nullable=True)
    org = relationship("Orgnization")
    is_active = db.Column(db.Boolean)
    active_at = db.Column(db.DateTime)
    login_at = db.Column(db.DateTime)
    created_at = db.Column(db.DateTime, default=datetime.now())

    @classmethod
    def make_hashed_passwd(cls, passwd):
        salt = current_app.config.get("PASSWD_SALT")
        content = f'{salt}{passwd}'
        h = hashlib.md5(content.encode())
        return h.hexdigest()

    @classmethod
    def is_exists(cls, name, phone_num):
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
            User.phone_num == name_or_phone_num,
        )
        query = User.query.filter(condition).filter(
            User.hashed_passwd == passwd)
        return query

    @classmethod
    def load_user(cls, user_id):
        return User.query.filter(User.id == user_id).one_or_none()

    def get_id(self):
        return "{}".format(self.id)

    @property
    def is_authenticated(self):
        return True

    def is_admin(self):
        return self.has_role('admin')

    def has_role(self, *role_names):
        """
        check if user has one of role_names
        """
        query = db.session.query(UserRole).join(Role,
                                                UserRole.role_id == Role.id)
        query = query.filter(UserRole.user_id == self.id,
                             Role.name.in_(*role_names)).exists()
        return db.session.query(query).scalar()

    @property
    def roles(self):
        role_ids = [item.role_id for item in UserRole.query.filter()]
        roles = Role.query.filter(Role.id.in_(role_ids))
        return roles


class UserRole(db.Model):
    """
    every user has many roles
    """
    id = db.Column(db.Integer, primary_key=True)
    user_id = db.Column(db.Integer)
    role_id = db.Column(db.Integer)


class Orgnization(db.Model):
    """
    hospital: 医院
    """
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(200), unique=True)
    kind = db.Column(db.String(100))
    address = db.Column(db.String(400))
    created_at = db.Column(db.DateTime)
    province = db.Column(db.String(20))
    city = db.Column(db.String(30))
    introduction = db.Column(db.Text)

    @classmethod
    def is_exists(cls, name):
        """
        check if user with name or phone_num
        has already exists.
        """
        query = Orgnization.query.filter(Orgnization.name == name).exists()
        return db.session.query(query).scalar()


class Role(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    name = db.Column(db.String(100), unique=True)
    perms = relationship("RolePerm", back_populates="role")


class RolePerm(db.Model):
    """
    every role can has many perm codes
    """
    id = db.Column(db.Integer, primary_key=True)
    role_id = db.Column(db.Integer, ForeignKey("role.id"))
    perm_name = db.Column(db.String(50))
    role = relationship("Role", back_populates="perms")


def create_admin(name, passwd):
    hashed_passwd = User.make_hashed_passwd(passwd)
    user = User(name=name, hashed_passwd=hashed_passwd)
    user.is_active = True
    admin_role = Role(name="admin")
    role = Role.query.filter(Role.name == "admin").one_or_none()
    if not role:
        role = Role(name="admin")
        db.session.add(role)
    db.session.add(user)
    db.session.commit()
    user_role = UserRole(role_id=role.id, user_id=user.id)
    db.session.add(user_role)
    db.session.commit()
