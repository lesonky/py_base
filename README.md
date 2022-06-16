# Base

公司公共代码

代码结构如下

```
├── README.md
├── ai # AI 模型训练和预测代码
│   ├── pred  # 生产环境预测代码
│   │   └── Dockerfile
│   ├── sdk # 封装的tensorRT, libtorch
│   └── train # 训练代码
│       └── Dockerfile
├── deploy # 部署脚本
│   ├── docker # 使用docker cm部署
│   │   ├── cm # config map
│   │   └── services # 各种微服务的docker compose配置
│   ├── k8s # k8s 部署脚本
│   └── scripts # 同步脚本
├── docs # 项目设计文档
├── fe # 前端
│   └── Dockerfile # 
└── service # 后端服务
    ├── py # python 后端
    │   └── Dockerfile
    └── rs # rust后端
        └── Dockerfile
```

# 前端开发
VSCode 请以 fe 目录为根目录,保证插件的正常运行

请使用 `yarn` 进行包管理

运行 `yarn` 安装依赖

运行 `yarn dev` 开启开发服务器

修改代理服务器,请修改`fe/config/proxy.ts` 文件 
请使用`git update-index --skip-worktree fe/config/proxy.ts` 命令,避免代理服务器配置被git追踪


# 后端开发

进入py 目录环境

```bash
cd ./service/py/
```

安装pipenv，以及python依赖包

```bash
pip install pipenv
pipenv shell
pipenv install
```

本地开发,`FlASK_ENV` 指定用的配置文件， 使用configs/development.py为配置文件

```bash
pipenv shell
FLASK_ENV=development FLASK_APP=app/run.py flask run
```

数据库schema升级, 自动生成migration 文件

```bash
FLASK_ENV=development FLASK_APP=app/run.py flask db migration -m "init database schema"
```

使用migration文件，升级数据库

```bash
FLASK_ENV=development FLASK_APP=app/run.py flask db upgrade
```
创建管理员

```bash
FLASK_ENV=development FLASK_APP=app/run.py flask user_admin create_admin user_name passwd
```

pytest 单元测试
```bash
py.test
```

# docker-compose 部署

部署到测试服务器，这个地方请参照 `./deploy/docker/env_test.sh` 将里面一些变量改为其他的

```bash
$cp ./deploy/docker/env_test.sh ./deploy/docker/env_test2.sh
#修改env_test2.sh 中的配置
$DEPLOY_ENV=env_test2.sh ./deploy/docker/deploy.sh
```

为了方便部署，可以将ssh-key 拷贝到服务器上, 将`xieyu@192.168.0.14`改为自己的
```
ssh-copy-id -i  ~/.ssh/id_rsa.pub xieyu@192.168.0.14
```
为了避免每次运行docker要sudo, 可以请有sudo 权限的用户，将自己加入到docker组, 将下面的`$USER`改为自己的用户名

```
sudo gpasswd -a $USER docker
```

# K8s 部署
