#!/usr/bin/env python2

"""This module defines the API class coachbot which is exposed to the
usercode.
"""

from cocos_py2 import cocos
import time
import logging

class Coachbot:
    """Represents the base Coachbot. Currently, there is no simple way to
    either compose this class nor inherit from it. This is the only interface
    to the Coachbot available.
    """

    def __init__(self, communicator):
        # type: (cocos.CocosCommunicator) -> None
        self._id = -1  # type: int
        self.__cocos = communicator
        logging.basicConfig()  # TODO: Remove and replace with IPC logging

    @property
    def id(self):  # pylint: disable=invalid-name
        # type: () -> int
        """
        Facility for fetching the identification number of the current robot.

        You can use this property to return the current robot id, for example:

        .. code-block:: python

            # Set the color of bot 3 to red and others to green.
            robot.set_led(*((100, 0, 0) if robot.id == 3 else (0, 100, 0)))

        Returns:
            int: The id number of self.
        """
        return self._id

    @property
    def logger(self):
        return logging.getLogger('api.v1.user')

    @id.setter
    def id(self, new):  # pylint: disable=invalid-name
        # type: (int) -> None
        self.logger.warn('Attempting to set id to %d. This is unsupported '
                         'behavior. Ignored.', id)

    def set_led(self, r, g, b):  # pylint: disable=invalid-name
        # type: (int, int, int) -> None
        """Sets the color of the onboard LED.

        Note:
            This function **does not accept values between 0-255**. Allowable
            values are between 0 - 100.

        Parameters:
            r (int): red value (0 - 100).
            g (int): green value (0 - 100).
            b (int): blue value (0 - 100).
        """
        try:
            self.__cocos.send_led((r, g, b))
        except ValueError as v_err:
            self.logger.exception(v_err)

    def set_vel(self, left, right):
        # type: (int|float, int|float) -> None
        """
        Sets the speed for the left and right wheel in percentage values.

        Parameters:
            left (int): The left motor speed (-100 - 100)
            right (int): The right motor speed (-100 - 100)
        """
        try:
            self.__cocos.send_vel((int(left), int(right)))
        except ValueError as v_err:
            self.logger.exception(v_err)

    def get_clock(self):
        # type: () -> float
        """
        Returns:
            float: The time elapsed since the program started in seconds.
        """

    def send_msg(self, msg):
        # type: (str) -> bool
        """Attempts to transmit the given message returning whether it was
        successful.

        Parameters:
            msg (str): The message to attempt to transmit. This message must be
            of size ``coach_os.custom_net.MSG_LEN - 8`` or shorter. Longer
            messages are trimmed. The ``-8`` is here due to being a legacy bug.
        """

    def recv_msg(self, clear=False):
        # type: (bool) -> list[str]
        """
        Reads up to ``custom_net.talk.MAX_MSG_NUM`` messages since the last
        invokation. If this function does not have any new updates to send, it
        will return an empty list.

        Parameters:
            clear (bool): Whether to clear the message buffer after reading.

        Returns:
            list[str]: Up to ``custom_net.MAX_MSG_NUM`` messages since last
            invokation.
        """

    def get_pose(self):
        # type: () -> tuple[float, float, float]
        """
        This function retrieves the pose of the robot, if it can. If it can't
        it returns None.

        Returns:
            tuple[float, float, float] | None: The global pose as a tuple (x,
            y, theta) if new data available since last invokation, None
            otherwise.
        """
        try:
            return self.__cocos.send_get_position()
        except ValueError as v_err:
            self.logger.exception(v_err)

    def delay(self, millis=200):
        # type: (float) -> None
        """Waits some miliseconds (default 200).

        Parameters:
            millis: The amount of time to wait.

        Todo:
            Handle better the complicated exception that is raised.
        """
        time.sleep(float(millis) / 1000.0)  # TODO: Is this ok?
