from flask_sqlalchemy import model

from app.run import app, db


class BaseTest:
    mimetype = 'application/json'
    headers = {'Content-Type': mimetype, 'Accept': mimetype}

    def admin_login(self, client):
        data = {'name_or_phone_num': 'admin', 'passwd': 'webace'}
        resp = client.post('/api/user/login', json=data)
        print("the resp is", resp.json)
        assert resp.status_code == 200

    @classmethod
    def setup_class(cls):
        with app.app_context():
            from app.models.user import create_admin
            db.create_all()
            create_admin(name='admin', passwd='webace')

    @classmethod
    def teardown_class(cls):
        with app.app_context():
            db.drop_all()

    def setup_method(self, method):
        pass

    def teardown_method(self, method):
        #exclude_tables = []
        # models = {
        #    m.__tablename__: m
        #    for m in db.Model._decl_class_registry.values()
        #    if isinstance(m, model.DefaultMeta)
        # }
        #tables = db.metadata.sorted_tables
        # tables.reverse()
        # with app.app_context():
        #    for table in tables:
        #        if table.name in exclude_tables:
        #            continue
        #        db.session.query(models[table.name]).delete()
        #        db.session.commit()
        pass
