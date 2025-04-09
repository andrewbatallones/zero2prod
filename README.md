# Zero 2 Prod

This project is following the [Zero 2 Prod](https://www.zero2prod.com/index.html?country_code=US) book

## Initial Setup

### Direnv

You will need to have [divenv](https://direnv.net/) in order to get the necessary environment variables loaded. Afterwards, you can run this command to load the needed ENV_VARS.

```bash
direnv allow
```

### Database Setup

You will need to have a instance of PostgreSQL running. There is a compose.yaml file that will generate one or you can set one up locally. Afterwards, you will need to create the user and password.

Install SQLx CLI:

```bash
cargo install sqlx-cli --no-default-features --features native-tls,postgres
```

If you ended up not using the Docker iamge provided, you'll probably either need to add in your own user/password credentials, or you can run this command that will manually create the user.

```bash
psql postgres://username:password@localhost:5432 < scripts/structure.sql
```

From there, you can run the database init command.

```bash
./scripts/init_db.sh
```