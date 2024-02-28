# Zero 2 Prod

This project is following the [Zero 2 Prod](https://www.zero2prod.com/index.html?country_code=US) book

## Initial Setup

You will need to have [divenv](https://direnv.net/) in order to get the necessary environment variables loaded.

This command will initialize the database on Docker so that the app can connect to it.

```
./scripts/init_db.sh
```

This command will load the environment variables to the service

```
direnv allow
```
