## Viewing database tables in Docker
Exec into the container. Once in the container, run the following to sign in as user `postgres`
```
psql -U postgres
```
Then change to the `newsletter` database
```
\c newsletter
```
To view tables, run
```
\dt
```
