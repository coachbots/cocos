#!/usr/bin/env python2

from __future__ import absolute_import, print_function
import sys
from cocos_py2.api import Coachbot
from cocos_py2.cocos import CocosCommunicator

USER_CODE_TEMPLATE = \
"""
import sys

{user_script}

usr(bot)
"""

class App:
    def __init__(self):
        # type: () -> None
        self.run_script = ''  # type: str

    def populate_script(self, script):
        # type: (str) -> None
        self.run_script = USER_CODE_TEMPLATE.format(user_script=script)

    def run(self):
        # type: () -> None
        cocos = CocosCommunicator(sys.argv[1])
        cocos.begin()
        coachbot = Coachbot(cocos)
        exec(self.run_script, { 'bot': coachbot })

TEST_SCRIPT = \
"""
def usr(bot):
    while True:
        bot.set_vel(100, 100)
"""

def main():
    # type: () -> int
    app = App()
    app.populate_script(TEST_SCRIPT)
    app.run()
    return 0


sys.exit(main())
