from flask import Blueprint, jsonify

bp = Blueprint('tools', __name__)


@bp.route('/ping', methods=['GET'])
def ping():
    """ For health check.
    """
    return jsonify({'ping': 'ok'})
