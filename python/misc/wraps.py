from functools import wraps


def api_call(methods=None):
    if not methods:
        methods = ['GET']
    def decorator(f):
        @wraps(f)
        def wrapper(method=None, *args, **kwds):
            if method not in methods:
                raise RuntimeError(405)
            res = f(method, *args, **kwds)
            if not res:
                raise RuntimeError(404)
            return res
        return wrapper
    return decorator


@api_call(methods=['POST', 'PUT'])
def invalid_method(method):
    pass

@api_call()
def return_none(method):
    return None

@api_call()
def ok(method):
    return 99


for f in [invalid_method, return_none, ok]:
    try:
        print f.func_name, f(method='GET')
    except Exception, e:
        print 'Error %r' % e
