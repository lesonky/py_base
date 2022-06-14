import importlib

from flask import Flask, jsonify
from flask.helpers import get_env
from flask_cors import CORS

from marshmallow.exceptions import ValidationError

from app.exts import db, migrate, ma, hobbit, login_manager


def register_extensions(app):
    db.init_app(app)
    migrate.init_app(app, db)
    ma.init_app(app)
    hobbit.init_app(app, db)
    login_manager.init_app(app)
    migrate.init_app(app)


def register_blueprints(app):
    from app import views
    for name in views.__all__:
        bp = getattr(importlib.import_module(f'app.views.{name}'), 'bp', None)
        if bp is not None:
            app.register_blueprint(
                bp, url_prefix=f"/api{bp.url_prefix if bp.url_prefix else ''}")
    print(app.url_map)


def handle_marshmallow_validate_error(e: ValidationError):
    return jsonify({
        "code": 400,
        "message": e.messages,
    }), 400


def register_error_handler(app):
    from app.utils import result
    app.register_error_handler(ValidationError,
                               handle_marshmallow_validate_error)
    result.register_error_handler(app)


def register_cmds(app):
    pass


def create_app():
    app = Flask(__name__, instance_relative_config=True)
    app.config.from_object('app.configs.{}'.format(get_env()))

    with app.app_context():
        register_extensions(app)
        register_blueprints(app)
    register_error_handler(app)
    register_cmds(app)

    # 允许跨域访问
    CORS(app, supports_credentials=True)
    return app


app = create_app()

if __name__ == "__main__":
    app.run()
