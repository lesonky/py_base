from app.core import result
from app.exts import db
from app.exts import ma
from datetime import datetime
from sqlalchemy import Column, DateTime
from app.core.schema import DumpPageSchema


class BaseModelMixin():

    @classmethod
    def schema_addons(cls):
        return []

    @classmethod
    def schema(cls, **kwargs):

        class Schema(ma.SQLAlchemyAutoSchema, DumpPageSchema):

            class Meta:
                model = cls
                include_fk = True

        for addon in cls.schema_addons():
            print("apply schema addon", addon)
            addon(Schema)

        return Schema(**kwargs)

    @classmethod
    def get_query(cls):
        return db.session.query(cls)

    @classmethod
    def all(cls, *args):
        return db.session.query(cls).filter(*args).all()

    @classmethod
    def filter(cls, *args):
        return db.session.query(cls).filter(*args)

    @classmethod
    def session(cls):
        return db.session

    @classmethod
    def one_or_none(cls, *args):
        return db.session.query(cls).filter(*args).one_or_none()

    @classmethod
    def one_with_id_or_404(cls, id):
        obj = db.session.query(cls).filter(cls.id == id).one_or_none()  
        if obj is None:
            raise result.NotFound(f"{cls.__name__} with id {id} Not Found")
        return obj

    @classmethod
    def build_filter(cls, query_dict):
        filters = []
        for key, value in query_dict.items():
            if value in [None, ""]:
                continue
            key_parts = key.split("__")
            field, op_name = key_parts[0], "eq"
            if len(key_parts) == 2:
                op_name = key_parts[1]

            field = getattr(cls, field)
            exp = BaseModelMixin.to_exp(field, value, op_name)
            filters.append(exp)
        return filters

    @staticmethod
    def to_exp(key, value, op_name):
        if op_name == "eq":
            return key == value

        if op_name == "gt":
            return key > value

        if op_name == "lt":
            return key < value

        if op_name == "in":
            return key.in_(value)

        if op_name == "icontain":
            return key.ilike(f"%{value}%")

        if op_name == "contain":
            return key.like(f"%{value}%")

        if op_name == "is_":
            return key.is_(value)

    @classmethod
    def inject_foregin_schema(cls, items, name, ref_class, ref_id_field_name):
        if not isinstance(items, list):
            items = [items]
        ref_ids = [item[ref_id_field_name]
                   for item in items if bool(item[ref_id_field_name])]
        ref_map = {}

        for item in items:
            key = item[ref_id_field_name]
            if bool(key):
                ref_map[key] = ref_map.get(key, [])
                ref_map[key].append(item)

        query = ref_class.filter(ref_class.id.in_(ref_ids))
        ref_data = ref_class.schema(many=True).dump(query)

        for ref_item in ref_data:
            items = ref_map.get(ref_item['id'], [])
            for item in items:
                item[name] = ref_item
        return items


class TimeMixin(object):
    created_at = Column(DateTime(), default=datetime.now)
    updated_at = Column(DateTime(), default=datetime.now)
