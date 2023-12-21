# rust-sql-test
Test out a Rust project and various SQL setups

## DB

### DB setup

```
rm -rf $PGDATA
pg_ctl init
make services
```

```
sh ./db_init.sh
```

