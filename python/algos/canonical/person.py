class Person(object):

    def __init__(self, id, displayname, members=None):
        self.id = id
        self.displayname = displayname
        self.members = members
        self.is_team = members is not None
