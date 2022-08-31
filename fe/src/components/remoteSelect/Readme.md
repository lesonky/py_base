# 说明

- 用于从接口加载选项的 Select 组件
- 与 a-select 保持用法一致
- 新增参数
  - type : 下拉选择的内容
  - useCache : 是否使用缓存
- 如果需要新增一个 type ,只要在文件 [`src/api/dictionary.ts`](src/api/dictionary.ts) 中定义一个函数,类型会根据文件自动推断
- 函数返回值必须和其他返回值保持一致
- 函数会在 [`src/store/modules/dictionary/index.ts`](src/store/modules/dictionary/index.ts)中被使用,不需要手动处理

# TODOs

- 适配多选
