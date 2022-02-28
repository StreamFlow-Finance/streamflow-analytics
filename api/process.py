def process(data, args):
    c = CollectionProcessor(data)
    filter_field = args.get('filter_field')
    filter_operand = args.get('filter_op')
    filter_value = args.get('filter_val')

    if not (filter_value is None or filter_operand is None or filter_field is None):
        c = c.filter(filter_field, filter_operand, filter_value)

    order_by = args.get('order_by')
    order_asc = args.get('order_dir') == 'asc'

    if order_by is not None:
        c = c.order_by(order_by, order_asc)
    return c.data


class CollectionProcessor:

    def __init__(self, data):
        self.data = data

    def order_by(self, field, asc=True):
        try:
            return self.__class__({k: v for k, v in sorted(self.data.items(), key=lambda item: item[1].get(field), reverse=not asc)})
        except TypeError:
            return self.data

    def filter(self, field, operand, value):
        try:
            value = int(value)
        except:
            pass

        def expr(field, operand, value):
            if operand == 'eq':
                return field == value
            elif operand == 'ne':
                return field != value
            elif operand == 'g':
                return field > value
            elif operand == 'l':
                return field < value
            elif operand == 'ge':
                return field >= value
            elif operand == 'le':
                return field <= value
            return False
        return self.__class__({k: v for k, v in self.data.items() if expr(v.get(field), operand, value)})