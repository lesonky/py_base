# 后端开发

## 安装 pipenv，以及 python 依赖包

```bash
pip install pipenv  # 安装 pipenv
pipenv shell        # 初始化虚拟环境
pipenv install      # 安装 pip 包
```

## 指定项目配置文件

- 本地开发,`FlASK_ENV` 指定用的配置文件,默认值为`production` 会使用 `configs/default.py`作为配置文件.
- 本地开发时,请指定`FlASK_ENV`的值为`development`,使用`configs/development.py`为配置文件.

## 使用 pipenv script 启动项目

**可以使用 pipenv script 来启动本地开发(windows 机器可能无法使用,请查看下面)**

```bash
pipenv run dev  # 启动开发环境

# 修改 models 后执行，升级数据库
pipenv run migration # 生成数据库升级脚本,在 migrations 文件夹中
pipenv run dbupgrade # 升级数据库

# 如果 migration 出现问题 Error: Can't locate revision identified by '156c57aa4682'
# 156c57aa4682 是一个版本号,存在数据库的 alembic_version 表里面
# 尝试使用下面的命令修复版本号, **注意修改命令中的版本号**
export FLASK_ENV=development
export FLASK_APP=app/run.py
flask db revision --rev-id 30dc7f6b846a

# 也可以直接去数据库里面找到对应的表,把对应的升级记录删除掉

# 初始化管理员
pipenv run gadmin
```

## 使用原生 shell 来启动开发环境

### 本地运行

```bash
pipenv shell
FLASK_ENV=development FLASK_APP=app/run.py flask run
```

### 数据库 schema 升级, 自动生成 migration 文件

```bash
FLASK_ENV=development FLASK_APP=app/run.py flask db migrate -m "init database schema"
```

### 使用 migration 文件，升级数据库

```bash
FLASK_ENV=development FLASK_APP=app/run.py flask db upgrade
```

### 创建管理员(注意,修改 user_name 和 passwd)

```bash
FLASK_ENV=development FLASK_APP=app/run.py flask user create_admin user_name passwd
```

### pytest 单元测试

```bash
py.test
```

## docker-compose 部署(请到项目根目录操作)

部署到测试服务器，这个地方请参照 [`./deploy/docker/env_test.sh`](../../deploy/docker/deploy.sh) 将里面一些变量改为其他的

```bash
$cp ./deploy/docker/env_test.sh ./deploy/docker/env_test2.sh
#修改env_test2.sh 中的配置
$DEPLOY_ENV=env_test2.sh ./deploy/docker/deploy.sh
```

**其他说明**

- 后端部署的端口在[services/web.yml](../../deploy/docker/services/web.yml)中
- 后端部署的数据库配置在[web_api/production.py](../../deploy/docker/cm/test/web_api/production.py)中,**注意:**,和开发环境不一样
- 前端部署后使用`nginx`和后端交互,`nginx的配置文件在` [fe/defalut.conf](../../deploy/docker/cm/test/fe/default.conf)中

## 其他说明

为了方便部署，可以将 ssh-key 拷贝到服务器上, 将`xieyu@192.168.0.14`改为自己的

```
ssh-copy-id -i  ~/.ssh/id_rsa.pub xieyu@192.168.0.14
```

为了避免每次运行 docker 要 sudo, 可以请有 sudo 权限的用户，将自己加入到 docker 组, 将下面的`$USER`改为自己的用户名

```
sudo gpasswd -a $USER docker
```

## K8s 部署

# 接口开发

## 新增业务表

1. 在`app/models/`目录下新增一个文件,并定义表结构,示例代码如下

```python
from sqlalchemy import Column, Integer, String, FLOAT, TEXT, DateTime
from app.core.model import BaseModelMixin, TimeMixin
from app.exts import db


class Patient(db.Model, BaseModelMixin, TimeMixin):
    """
    病人信息表
    """
    __tablename__ = 'tb_patient'

    id = Column(Integer(), primary_key=True, comment='自增主键')
    medical_id = Column(String(256), comment='病历号')
    name = Column(String(256), comment='姓名')
    sex = Column(Integer(), comment='性别')
    date = Column(DateTime, comment='出生年月')
    note = Column(TEXT, default='', comment='描述')
```

2. 运行数据库升级脚本生成命令`pipenv run migration`
3. 运行数据库升级命令`pipenv run dbupgrade`

## 基础模型说明

### TimeMixin 为模型添加 `create_at` 和 `update_at` 两个字段

### BaseModelMixin 为模型添加了一些基础工具方法

- schema: 通过 SQLAlchemyAutoSchema 为模型自动生成 schema 减少 schema 定义工作量
- get_query: 获取数据库 query 对象
- all: 查询所有记录
- filter: 设置查询参数(注意,这里不会获取结果)
- one_or_none: 获取数据,如果不存在则放回 None
- one_with_id_or_404:通过 id 来获取数据,如果不存在,则报错
- build_filter: 通过一个外部 dict 来构建查询条件
- to_exp: 静态方法,build_filter 时,用来解析 dict 的 key,并设置为相应的条件
- inject_foregin_schema: 设置外键关联的对象(这里是一次将所需要的外连数据全查出来,在通过 ID 对应设置到对象上,效率要高一点)

### build_filter 介绍

- 用来做条件的 dict 通过 key\_\_option 的形式来命名
- key 对应 Model 的字段名称
- option 对应查询条件的操作
  - eq 等于(默认)
  - gt 大于
  - lt 小于
  - in 包含于
  - icontain 包含(忽略大小写)
  - contain 包含
  - is\_
- 例如查询条件为 {"name\_\_icontain":"三"} 最终会编译为 class.name.ilike("%三%")
