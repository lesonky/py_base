from flask import Blueprint, jsonify

from app.utils.result import BadParam, ok
from app.utils import result

bp = Blueprint('example', __name__)


def check():
    raise result.BadParam("BadParam")


@bp.route('/example/bad_param', methods=['GET'])
def example_bad_param():
    """ For health check.
    """
    check()
    return result.ok()


@bp.route('/example/ok', methods=['GET'])
def example_ok():
    """ For health check.
    """
    return result.ok({"data": "hello, world"})


@bp.route('/example/empty_data_ok', methods=['GET'])
def example_empty_data():
    """ For health check.
    """
    return result.ok()


@bp.route('/example/exception', methods=['GET'])
def example_excpetion():
    """ For health check.
    """
    raise Exception("hello, world")
