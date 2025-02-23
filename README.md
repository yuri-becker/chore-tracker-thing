[![AGPL-3.0 License](https://img.shields.io/github/license/yuri-becker/chore-tracker-thing?style=for-the-badge&logo=gnu&logoColor=white&color=%23A42E2B )](https://github.com/yuri-becker/chore-tracker-thing/blob/develop/LICENSE.md)
[![Latest Commit](https://img.shields.io/github/last-commit/yuri-becker/chore-tracker-thing/develop?style=for-the-badge)](https://github.com/yuri-becker/cchore-tracker-thing/commits/develop)
[![Coveralls](https://img.shields.io/coverallsCoverage/github/yuri-becker/chore-tracker-thing?style=for-the-badge&branch=develop)](https://coveralls.io/github/yuri-becker/chore-tracker-thing)


<br />
<div align="center">

  <h1 align="center"><strong>Chore Tracker Thing</strong></h1>

  <p align="center">
    (WIP) Self-hostable, multi-user- and multi-tenant-capable application to manage household chores.
  </p>
</div>
<br/>

## About the Project

This project began with us looking for an open-source solution to track household chores, that was both simple to use
and still versatile enough, needing something that

* is mobile-first,
* doesn't get in the way,
* is focused around recurring tasks,
* is lenient about when tasks are completed (depression-friendly), and
* supports multiple users per household and multiple households per user .

None of the existing solutions fit our needs, so we decided to develop our own – Chore-Tracker-Thing.

Chore-Tracker-Thing is trying to achieve exactly what we needed and is designed to be self-hosted with an
OIDC-compatible authentication backend like Authelia.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Built With

[![Rust](https://img.shields.io/badge/Rust-20232A?style=for-the-badge&logo=rust&logoColor=FFFFFF)](https://www.rust-lang.org)
[![Rocket](https://img.shields.io/badge/Rocket-20232A?style=for-the-badge&logo=rocket&logoColor=D33847)](https://rocket.rs)
[![MongoDB](https://img.shields.io/badge/Postgres-20232A?style=for-the-badge&logo=postgresql&logoColor=4169E1)](https://www.postgresql.org/)

[![Vite](https://img.shields.io/badge/Vite-20232A?style=for-the-badge&logo=vite&logoColor=646CFF)](https://vitejs.dev)
[![React](https://img.shields.io/badge/React-20232A?style=for-the-badge&logo=react&logoColor=61DAFB)](https://react.dev/)
[![TypeScript](https://img.shields.io/badge/TypeScript-20232A?style=for-the-badge&logo=typescript&logoColor=3178C6)](https://www.typescriptlang.org/)
[![yarn](https://img.shields.io/badge/yarn-20232A?style=for-the-badge&logo=yarn&logoColor=2C8EBB)](https://yarnpkg.com/)
[![Mantine](https://img.shields.io/badge/Mantine-20232A?style=for-the-badge&logo=mantine&logoColor=339AF0)](https://mantine.dev/)

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Usage

### Environment Variables

| Name                          | Description                                                                                                 | Default     |
|-------------------------------|-------------------------------------------------------------------------------------------------------------|-------------|
| `CHORES_HOST`                 | The URL under which the app will be available.                                                              | *required*  |
| `CHORES_SECRET`               | Server-side secret for encrypting cookies. Set to something random. Generate with `openssl rand -base64 32` | *required*  |
| `CHORES_PORT`                 | Port under which the backend should run.                                                                    | `8001`      |
| `CHORES_MODE`                 | `debug` or `prod`                                                                                           | `prod`      |
| `CHORES_POSTGRES_HOST`        | Host on which Postgres runs.                                                                                | `127.0.0.1` |
| `CHORES_POSTGRES_PORT`        | Port on which Postgres runs.                                                                                | `5432`      |
| `CHORES_POSTGRES_USER`        | User to use for the Postgres connection.                                                                    | *required*  | 
| `CHORES_POSTGRES_PASSWORD`    | Password to use for the Postgres connection                                                                 | *optional*  | 
| `CHORES_POSTGRES_DATABASE`    | Postgres database to use.                                                                                   | *required*  |
| `CHORES_OIDC_ENDPOINT`        | URL on which the OIDC Provider runs on.                                                                     | *required*  |                             
| `CHORES_OIDC_CLIENT_ID`       | OIDC Client ID for this application                                                                         | *required*  |
| `CHORES_OIDC_CLIENT_PASSWORD` | OIDC Client Secret for this application                                                                     | *required*  |

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## Development

### Prerequisites

* [NodeJS](https://nodejs.org/en)
* [yarn](https://yarnpkg.com/getting-started/install)
* [docker](https://www.docker.com/)
* For Backend development [Rust & Cargo](https://www.rust-lang.org/tools/install), but it can also be run in Docker if
  you don't plan to make any code changes.
* Any IDE of your choice. Project ships with configuration for Jetbrains IDEs.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Set-up

Install the [Prerequisites](#prerequisites) and clone the project.

If you need to override any [Environment Variables](#environment-variables), create a `.env.local` and set those in there. You would only need this if you,
for example, want to test with a production authentication provider, or you don't want to run postgres via docker compose.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

### Running

To run the frontend use

```sh
cd web
yarn dev
```

Vite proxies `/api` and `/oidc` to the backend.

If you want to run the database and backend via Docker (recommended if you don't want to do code changes in there), use

```sh
docker compose up
```

Otherwise, you can run only the database and authentication endpoint (dex) via

```sh
docker compose up postgres dex
```

and then run the backend with

```sh
cargo run
```

### Credentials

There are two users, both have the password `123`, for the e-mail address, enter either just `a` or `b`.

<p align="right">(<a href="#readme-top">back to top</a>)</p>

## License

Distributed under the terms of the GNU Affero General Public License, Version 3. See [LICENSE.md](/LICENSE.md) for the
exact terms.

<p align="right">(<a href="#readme-top">back to top</a>)</p>
