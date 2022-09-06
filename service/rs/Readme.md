# 工具安装

安装cargo-watch 用于后端live reload

```
cargo install cargo-watch
```

安装 sqlx-cli, 用于数据库migrate

```
cargo install sqlx-cli
```

# 配置

相关用dotenv来配置，保存在.env中
```
HOST=127.0.0.1
PORT=8001
RUST_LOG=webace-http-base=DEBUG
DATABASE_URL=mysql://webace_base_rs:webace@192.168.0.14:3308/webace_base_rs
JWT_SECRET=sLxFEgxVbRRRXNz9q9zQKmyiGhT9EKP6QRNZgdogBACYqeWafo8QLoUjyvQxTM26
PASSWD_SALT=3GLYgAJZYxmpLHu7MXLPBJAXQspGJMNaq111QGema61Q8P48naQ0FXXLYHTEDZDYosftn
AVATAR_ROOT=/opt/t/webace_base_rs/avatar
```

# 数据库升级

生成migrate文件
```
sqlx migrate add create-user-table
```

跑数据库升级sql
```
sqlx migrate run
```

sqlx 升级记录会保存在`_sqlx_migrations`文件中, 如果升级失败了，则需要手动删除相应升级记录，
修改升级sql脚本，然后重新run, 比如如下记录会删除vversion 20220905075334的升级记录。

```
delete from _sqlx_migrations where version=20220905075334
```


# 本地开发

RUST_LOG=DEBUG，打开log日志，cargo watch -d 30 表示当rust代码有修改时，延迟30s 触发`cargo run -r`命令。

```
RUST_LOG=DEBUG cargo watch -d 30 -x 'run -r'
```
