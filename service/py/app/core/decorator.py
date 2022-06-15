from functools import wraps

from flask import current_app, g
#from flask_login import current_user
from flask_jwt_extended import current_user

from app.core.result import NoPermission


def roles_accepted(*role_names):
    """| This decorator ensures that the current user is logged in,
    | and has *at least one* of the specified roles (OR operation).

    Example::

        @route('/edit_article')
        @roles_accepted('Writer', 'Editor')
        def edit_article():  # User must be 'Writer' OR 'Editor'
            ...

    | Calls unauthenticated_view() when the user is not logged in
        or when user has not confirmed their email address.
    | Calls unauthorized_view() when the user does not have the required roles.
    | Calls the decorated view otherwise.
    """

    # convert the list to a list containing that list.
    # Because roles_required(a, b) requires A AND B
    # while roles_required([a, b]) requires A OR B
    def wrapper(view_function):

        @wraps(view_function)  # Tells debuggers that is is a function wrapper
        def decorator(*args, **kwargs):
            if not current_user.has_roles(role_names):
                raise NoPermission(
                    msg=
                    f"no permission, require use have one of role: {role_names}"
                )
            # It's OK to call the view
            return view_function(*args, **kwargs)

        return decorator

    return wrapper


def roles_required(*role_names):
    """| This decorator ensures that the current user is logged in,
    | and has *all* of the specified roles (AND operation).

    Example::

        @route('/escape')
        @roles_required('Special', 'Agent')
        def escape_capture():  # User must be 'Special' AND 'Agent'
            ...

    | Calls unauthenticated_view() when the user is not logged in
        or when user has not confirmed their email address.
    | Calls unauthorized_view() when the user does not have the required roles.
    | Calls the decorated view otherwise.
    """

    def wrapper(view_function):

        @wraps(view_function)  # Tells debuggers that is is a function wrapper
        def decorator(*args, **kwargs):
            # User must have the required roles
            if not current_user.has_roles(*role_names):
                raise NoPermission(
                    msg=
                    f"no permission, require use have all role of: {role_names}"
                )
            # It's OK to call the view
            return view_function(*args, **kwargs)

        return decorator

    return wrapper
