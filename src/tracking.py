import datetime as dt
from pathlib import Path
from typing import Dict, List

import click
from yaml import Loader, dump, load


def root_dir():
    present_dir = Path(".")
    root_dir = present_dir.parent.resolve()
    return root_dir


def file_path():
    file_path = Path("data/tracking.yml")
    full_path = root_dir() / file_path
    return full_path


def retrieve_file():
    with open(file_path()) as f:
        t = load(f, Loader=Loader)
    return t


def save_data_back_to_file(data: Dict) -> None:
    with open(file_path(), "w") as f:
        dump(data, f)


def create_time_entry(time_type: str) -> List:
    utc_time = dt.datetime.utcnow().strftime("%H:%M")
    time_entry = [time_type, utc_time]
    return time_entry


@click.command()
@click.argument("time_type")
def insert_time(time_type: str):
    time_type = time_type.lower()
    options = ["start", "stop"]
    if time_type not in options:
        raise Exception(f"Type needs to be in {options}")

    present_data = retrieve_file()
    data = present_data.copy()

    time_entry = create_time_entry(time_type)
    if "big" not in present_data.keys():
        present_data["big"] = []
    existing_time_entries = present_data["big"]
    existing_time_entries.append(time_entry)
    _ = save_data_back_to_file(data)

    print(present_data)


@click.group()
def group():
    pass


group.add_command(insert_time)

if __name__ == "__main__":
    # print(retrieve_file())

    group()
