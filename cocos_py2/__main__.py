#!/usr/bin/env python2

from __future__ import absolute_import, print_function
import sys
from cocos_py2.api import Coachbot
from cocos_py2.cocos import CocosCommunicator

USER_CODE_TEMPLATE = \
"""
import sys

{user_script}

usr(_bot)
"""

class App:
    def __init__(self):
        # type: () -> None
        self.run_script = ''  # type: str

    @staticmethod
    def format_script(script):
        # type: (str) -> str
        return USER_CODE_TEMPLATE.format(user_script=script)

    def run(self):
        # type: () -> None
        # The first argument must be the pipe through which to communicate with
        # cocos.
        cocos = CocosCommunicator(sys.argv[1])
        cocos.begin()
        coachbot = Coachbot(cocos)
        # Read the user script from stdin. Cocos will inject one.
        user_script = self.__class__.format_script(sys.stdin.read())
        exec(user_script, { '_bot': coachbot })


def main():
    # type: () -> int
    app = App()
    app.run()
    return 0


sys.exit(main())
