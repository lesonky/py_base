# 前端开发
VSCode 请以 fe 目录为根目录,保证插件的正常运行

请使用 `yarn` 进行包管理

运行 `yarn` 安装依赖

运行 `yarn dev` 开启开发服务器

修改代理服务器,请修改`fe/config/proxy.ts` 文件 
请使用`git update-index --skip-worktree fe/config/proxy.ts` 命令,避免代理服务器配置被git追踪

关于 labelStudio 的嵌入

请设置代理
```js
export default {
  '^/api/*': {
    target: 'http://192.168.0.14:9520',
  },
  '^/ls/*': {
    target: 'http://127.0.0.1:8080', // label-studio 本地运行环境,如果使用远程环境,这里也可以用 http://192.168.0.14:9520
  },
};
```
本地启动 labelStudio 到 8080 端口

**注意:** 必须使用 127.0.0.1 访问项目 不能使用 localhost ,否则嵌套页面无法正常显示

## 图标库说明
- 使用字节开源图标库 [IconPark](https://iconpark.oceanengine.com/official)
- 图标库已经全局安装,添加了前缀 'icon-park',例如,图标名称为 `whale` 则组件为 `<icon-park-whale>`

## windicss 使用
- 本项目采用 [windicss](https://cn.windicss.org/) 方案
- 大部分时候,采用框架自带样式就可以,如果有少许变动,用windicss可以很快添加样式,只需添加适当的 class ,对应的css代码会自动生成
  - 例如 x轴padding设置为20px,只需要添加 `px-20px` class 就可以,更多工具类,请[参考文档](https://cn.windicss.org/utilities/general/colors.html)

## husky 使用
- 本项目使用 husky 作为 git 提交检查工具
- git 提交前会检查文件格式,自动修复eslint,自动修复stylelint,检查 commit message
- 初始化husky会在第一次 yarn install 的时候进行,如果出错,请手动执行以下命令

```bash
cd .. && npx husky install && cd -
```

初始化结束之后,添加两个 hooks 需要手动执行
```bash
npx husky add ../.husky/pre-commit "cd fe && yarn lint-staged && cd -"
npx husky add ../.husky/commit-msg "cd fe && yarn commitlint --edit $1 && cd -"
```

