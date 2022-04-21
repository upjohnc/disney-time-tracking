import copy
import datetime as dt
from pathlib import Path
from pprint import pprint
from typing import Dict, List, Tuple

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


def get_datetime_now() -> Tuple[str, str]:
    datetime_now = dt.datetime.utcnow()
    date_str = datetime_now.strftime("%Y-%m-%d")
    utc_time = datetime_now.strftime("%H:%M")
    return date_str, utc_time


def create_time_entry(time_type: str, utc_time_str: str) -> List:
    time_entry = [time_type, utc_time_str]
    return time_entry


def pairwise(list_: List) -> List:
    a = iter(list_)
    return zip(a, a)


@click.command()
@click.argument("time_type")
def insert_time(time_type: str):
    time_type = time_type.lower()
    options = ["start", "stop"]
    if time_type not in options:
        raise Exception(f"Type needs to be in {options}")

    date_str, utc_time_str = get_datetime_now()

    present_data = retrieve_file()
    data = copy.deepcopy(present_data) if present_data is not None else {}

    time_entry = create_time_entry(time_type, utc_time_str)
    if date_str not in data.keys():
        data[date_str] = []
    existing_time_entries = data[date_str]
    existing_time_entries.append(time_entry)
    _ = save_data_back_to_file(data)

    print(data)


@click.command()
def show_file():
    present_data = retrieve_file()
    pprint(present_data)


def convert_to_datetime(time_str: str) -> dt.datetime:
    time_ = dt.datetime.strptime(time_str, "%H:%M")
    return time_


def time_difference(pair, print_line=False):
    if print_line:
        print(pair)
    start_time = pair[0]
    stop_time = pair[1]
    time_in_hours = (convert_to_datetime(stop_time[1]) - convert_to_datetime(start_time[1])) / dt.timedelta(hours=1)
    if print_line:
        print(time_in_hours)
    return time_in_hours


@click.command()
def check_file():
    present_data = retrieve_file()
    for date_, times in present_data.items():
        if len(times) == 1:
            print(date_, "Only one time for this date")
            continue
        paired_time = pairwise(times)
        comparison = [(first[0], second[0], first[0] != second[0]) for first, second in paired_time]
        print(date_, comparison)
        bad_entries = [i for i in comparison if i[2] is False]
        if bad_entries:
            print(f"     Bad entries: {bad_entries}")


@click.command()
def calculate_time():
    present_data = retrieve_file()
    for date_, times in present_data.items():
        print(date_)
        paired_time = pairwise(times)
        paired_hours = [time_difference(pair) for pair in paired_time]
        print(f"Hours for the day: {sum(paired_hours)}")


@click.command()
def calculate_time_detail():
    present_data = retrieve_file()
    for date_, times in present_data.items():
        print(date_)
        paired_time = pairwise(times)
        paired_hours = [time_difference(pair, print_line=True) for pair in paired_time]
        print(f"Hours for the day: {sum(paired_hours)}")


@click.group()
def group():
    pass


group.add_command(insert_time)
group.add_command(show_file)
group.add_command(calculate_time)
group.add_command(calculate_time_detail)
group.add_command(check_file)

if __name__ == "__main__":
    # print(retrieve_file())

    group()
