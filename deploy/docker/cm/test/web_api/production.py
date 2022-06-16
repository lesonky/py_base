from datetime import timedelta

DEBUG = True

# Change this for other project
JWT_SECRET_KEY = "sLxFEgxVbRRRXNz9q9zQKmyiGhT9EKP6QRNZgdogBACYqeWafo8QLoUjyvQxTM26"  # Change this!
JWT_ACCESS_TOKEN_EXPIRES = timedelta(hours=3 * 24)

PASSWD_SALT = '3GLYgAJZYxmpLHu7MXLPBJAXQspGJMNaq111QGema61Q8P48naQ0FXXLYHTEDZDYosftn'
SECRET_KEY = 'gWYplfrFgqAkWSWueUjGgjDWudGZsiZDKFirSa'

SQLALCHEMY_TRACK_MODIFICATIONS = False

SQLALCHEMY_DATABASE_URI = "mysql://webace_base:webace_base_123@192.168.0.14:3308/webace_base"

AVATAR_PATH = "/opt/data/avatar"
