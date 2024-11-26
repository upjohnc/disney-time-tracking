default:
    just --list

pre-commit:
    pre-commit install

poetry-install:
    poetry install --with dev --sync

add_time type:
    poetry run python src/tracking.py insert-time {{type}}

show:
    poetry run python src/tracking.py show-file

calculate:
    poetry run python src/tracking.py calculate-time

check:
    poetry run python src/tracking.py check-file

edit-file:
    nvim data/tracking.yml

docker_build:
    docker compose -f ./docker/docker-compose.yml build tracker

docker_run *args='':
    docker compose -f ./docker/docker-compose.yml run --rm tracker {{ args }}

# Add a start or end time to time tracker
docker_add *args='':
    docker compose -f ./docker/docker-compose.yml run --rm tracker insert-time {{ args }}
