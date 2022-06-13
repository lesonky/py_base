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