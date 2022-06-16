from enum import Enum, unique, auto

from app.core.result import BadParam


@unique
class Permission(Enum):
    All = 0
    # user related permissions
    EditUser = 1
    ViewUser = 2
    ListUser = 3
    UpsertUser = 4

    # role related permissions
    ListRole = 10
    CreateRole = 11
    EditRole = 12
    DeleteRole = 13
    ListPermission = 14

    def __str__(self):
        return self.name

    @classmethod
    def all_permission_name(cls):
        return [x.name for x in Permission]

    @classmethod
    def from_str_list(cls, names):
        try:
            ret = [Permission[x] for x in names]
            return ret
        except KeyError as e:
            raise BadParam(f"no permission found with name {str(e)}")


if __name__ == "__main__":
    a = Permission['EditUser']
    print(a.value, a.name)
    print(Permission(1))
    for x in Permission:
        print(x.name)
