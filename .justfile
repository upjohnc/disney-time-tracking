default:
    just --list

add_time type:
    poetry run python src/tracking.py insert-time {{type}}

show:
    poetry run python src/tracking.py show-file
