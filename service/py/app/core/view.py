def list_page(schema_cls, model_cls, args_location, order_by=None):
    args = schema_cls().load(args_location)
    page_num, page_size = schema_cls.pop_page(args)

    filters = model_cls.build_filter(args)
    query = model_cls.filter(*filters)
    if order_by is not None:
        query = query.order_by(order_by)
    total, items = model_cls.schema(many=True).dump_page(
        query, page_num, page_size)
    return total, items


def update_obj(obj, dic):
    for key, value in dic.items():
        setattr(obj, key, value)