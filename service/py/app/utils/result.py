from flask import jsonify
from sqlalchemy.sql.roles import ExpressionElementRole


def ok(result=None):
    data = {"success": True, "code": 0, "message": "ok", 'data': {}}
    if result:
        data['data'] = result
    return jsonify(data)


def err(code=400, message="", status_code=400):
    return jsonify({
        "success": False,
        "code": code,
        "message": message,
    }), status_code


class ErrWithCodeMsg(Exception):

    def __init__(self, status_code, code, msg):
        self.code = code
        self.msg = msg
        self.status_code = status_code


class BadParam(ErrWithCodeMsg):

    def __init__(self, msg):
        super().__init__(status_code=400, code=400, msg=msg)


class NoPermission(ErrWithCodeMsg):

    def __init__(self, msg):
        super().__init__(status_code=403, code=403, msg=msg)


class NotFound(ErrWithCodeMsg):

    def __init__(self, msg):
        super().__init__(status_code=404, code=404, msg=msg)


def handle_err_with_code_msg_err(e: ErrWithCodeMsg):
    return err(status_code=e.status_code, code=e.code, message=e.msg)


def handle_exception(e: Exception):
    # TODO: log the exeption stack for debug
    return err(status_code=500, code=500, message=str(e))


def register_error_handler(app):
    app.register_error_handler(ErrWithCodeMsg, handle_err_with_code_msg_err)
    app.register_error_handler(Exception, handle_exception)
