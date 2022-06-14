from datetime import datetime
import uuid
from app.models.user import create_admin
from app.exts import db
from functools import wraps
from flask_login import login_required
from flask import Blueprint, jsonify, request
import click
from app.utils.permission import roles_required

from marshmallow import Schema, fields

bp = Blueprint('user_admin', __name__)


@bp.route('/user_admin', methods=['GET'])
@login_required
@roles_required("admin")
def query_users():
    pass


@bp.route('/user_admin/active', methods=['POST'])
@login_required
@roles_required("admin")
def active_user():
    pass


@bp.cli.command('create_admin')
@click.argument('name')
@click.argument('passwd')
def cli_create_admin(name, passwd):
    create_admin(name, passwd)


class UserEditSchema(Schema):
    id = fields.Integer(required=True)
    name = fields.Str()
    phone_num = fields.Str()
    is_active = fields.Boolean()


@bp.route("/user_admin/edit", methods=['POST'])
@roles_required('admin')
def user_edit():
    pass
