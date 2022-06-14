import os

ROOT_PATH = os.path.split(os.path.abspath(__name__))[0]

DEBUG = True
SECRET_KEY = 'gWYplfrFgqAkWSWueUjGgjDWudGZsiZDKFirSa'
PASSWD_SALT = '3GLYgAJZYxmpLHu7MXLPBJAXQspGJMNaq111QGema61Q8P48naQ0FXXLYHTEDZDYosftn'
SQLALCHEMY_DATABASE_URI = 'sqlite:///{}'.format(
    os.path.join(ROOT_PATH, 'demo.db'))
SQLALCHEMY_TRACK_MODIFICATIONS = False
