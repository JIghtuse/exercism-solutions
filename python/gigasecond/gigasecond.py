import datetime


def add_gigasecond(current_datetime):
    return current_datetime + datetime.timedelta(seconds=1e9)
