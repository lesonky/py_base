from flask import jsonify
from flask_login import current_user
from functools import wraps
from enum import Enum


def roles_required(*role_name):
    """
    check if user has one of roles
    """

    def wrapper(func):

        @wraps(func)
        def decorated_view(*args, **kwarg):
            if not current_user.has_role(role_name):
                return jsonify({
                    "code": 401,
                    "message": f"role {role_name} required"
                }), 401
            return func(*args, **kwarg)

        return decorated_view

    return wrapper


class ExtendedEnum(Enum):

    @classmethod
    def list(cls):
        return list(map(lambda c: c.value, cls))
