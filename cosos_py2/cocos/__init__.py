from typing import Tuple


def notify_set_led(rgb):
    # type: (Tuple[int, int, int]) -> None
    for val in rgb:
        if val < 0 or val > 100:
            raise ValueError('Colors must be 0 <= color <= 100')
