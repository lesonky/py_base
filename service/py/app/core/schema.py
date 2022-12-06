from marshmallow import Schema, validate, fields, exceptions, pre_load

DateTimeField = fields.DateTime("%Y-%m-%d")


def camelcase(s):
    """ cover string from xxx_xxx to xxxXxx"""

    parts = iter(s.split("_"))
    return next(parts) + "".join(i.title() for i in parts)


def pop_prefix(args, prefix):
    """ remove prefix of args keys"""

    keys = list(args.keys())
    fields = {}
    for key in keys:
        if key.startswith(prefix):
            new_key = key[len(prefix):]
            fields[new_key] = args.pop(key, None)
    return fields


class CamelCaseSchemaMixin():
    """Schema that uses camel-case for its external representation
    and snake-case for its internal representation.
    """

    def on_bind_field(self, field_name, field_obj):
        field_obj.data_key = camelcase(field_obj.data_key or field_name)


class SeqListField(fields.List):

    def _deserialize(self, value, attr, data, **kwargs):
        try:
            return value.split(",")
        except AttributeError:
            raise exceptions.ValidationError(
                f"{attr} is not a list as not string")


class PageSchemaMixin():
    page_num = fields.Integer(missing=1, validate=validate.Range(min=1))
    page_size = fields.Integer(missing=10,
                               validate=validate.Range(min=1, max=100))

    @classmethod
    def pop_page(cls, args):
        page_num = args.pop('page_num')
        page_size = args.pop('page_size')
        return page_num, page_size


class QueryParamMixin:
    @pre_load(pass_many=False)
    def exclude_none(self, data, many, **kwargs):
        data = {key: value for key, value in data.items()
                if value is not None and value != ''}
        return data


class IdSchema(Schema):
    id = fields.Integer(required=True)


class DumpPageSchema():
    def dump_page(self, query, page_num, page_size):
        page = query.paginate(page=page_num,
                              per_page=page_size,
                              error_out=False)
        items = self.dump(page.items)
        total = page.total
        return total, items
