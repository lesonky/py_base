import os
from datetime import timedelta

ROOT_PATH = os.path.split(os.path.abspath(__name__))[0]

# Change this for other project
JWT_SECRET_KEY = "sLxFEgxVbRRRXNz9q9zQKmyiGhT9EKP6QRNZgdogBACYqeWafo8QLoUjyvQxTM26"  # Change this!
JWT_ACCESS_TOKEN_EXPIRES = timedelta(hours=3 * 24)

JWT_TOKEN_LOCATION = ["headers", "cookies"]
JWT_ALGORITHMS = "HS256"

#JWT_HEADER_NAME = "Token"
PASSWD_SALT = '3GLYgAJZYxmpLHu7MXLPBJAXQspGJMNaq111QGema61Q8P48naQ0FXXLYHTEDZDYosftn'
SECRET_KEY = 'gWYplfrFgqAkWSWueUjGgjDWudGZsiZDKFirSa'

DEBUG = True
SQLALCHEMY_DATABASE_URI = 'sqlite:///{}'.format(
    os.path.join(ROOT_PATH, 'demo.db'))
SQLALCHEMY_TRACK_MODIFICATIONS = False

AVATAR_PATH = "/opt/data/avatar"
