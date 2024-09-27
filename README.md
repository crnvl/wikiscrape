# wikiscrape
Wikiscrape is a datascience project that scrapes all Wikipedia articles and records their connections to other articles. The project is currently in development.

# Selfhost
You can simply run wikiscrape by yourself by using `docker-compose`. The following `docker-compose.yml` file will start a PostgreSQL database and the scraper service. The scraper service will scrape all Wikipedia articles and store them in the database. 

> Note: You might need to re-deploy these services when more services get added. Self-hosting is not recommended until the project is stable.

## Installation

1. Create a `docker-compose.yml` file in your project directory.
```yaml
services:
  database:
    image: 'postgres:latest'
    ports:
      - 5432:5432

    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: wikiscrape

    network_mode: "bridge"

    healthcheck:
      test: [ "CMD-SHELL", "pg_isready", "-U", "postgres" ]
      interval: 5s
      retries: 5

    volumes:
      - ${PWD}/db:/var/lib/postgresql/data


  scraper:
    image: "ghcr.io/crnvl/wikiscrape-scrape:latest"
    environment:
      DB_USERNAME: postgres
      DB_PASSWORD: wikiscrape
      DB_HOST: localhost
      DB_PORT: 5432

    network_mode: "host"

    depends_on:
      database:
        condition: service_healthy
```

2. Run `docker-compose up` in your project directory.

3. Profit.