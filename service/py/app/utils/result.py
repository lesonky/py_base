from flask import jsonify


def ok(result=None):
    data = {
        "success": True,
        "code": 0,
        "message": "ok",
    }
    if result:
        data['result'] = result
    return jsonify(data)


def err(code=400, message="", status_code=400):
    return jsonify({
        "success": False,
        "code": code,
        "message": message,
    }), status_code


class BadParam(Exception):

    def __init__(self, msg):
        self.msg = msg


def err_badparam(message=""):
    return err(code=400, message=message, status_code=400)
