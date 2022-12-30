# pgx-bug960

** Needs to be loaded via `shared_preload_libraries` in postgresql.conf **
```sql
CREATE EXTENSION bug960;
SELECT trigger_assertion_error();
```
