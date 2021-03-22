# Appendix: Schema for install log

## Installed package table

```sql
CREATE TABLE installed (
    date_time         DATETIME NOT NULL,
    package_set_group TEXT     NOT NULL,
    package_set       TEXT     NOT NULL,
    package           TEXT     NOT NULL,
    installer         TEXT     NOT NULL
);
```
