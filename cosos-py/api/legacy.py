#!/usr/bin/env python3

"""This module defines the API class coachbot which is exposed to the
usercode.
"""

class Coachbot:
    """Represents the base Coachbot. Currently, there is no simple way to
    either compose this class nor inherit from it. This is the only interface
    to the Coachbot available.
    """

    @property
    def id(self):  # pylint: disable=invalid-name
        # type: () -> int
        """
        Facility for fetching the identification number of the current robot.

        Warning:
            For legacy reasons, this property is settable, however, you should
            never do this! **Under no circumstance** should you be modifying
            this variable in user code.

        You can use this property to return the current robot id, for example:

        .. code-block:: python

            # Set the color of bot 3 to red and others to green.
            robot.set_led(*((100, 0, 0) if robot.id == 3 else (0, 100, 0)))

        Returns:
            int: The id number of self.
        """
        return self._id

    @id.setter
    def id(self, new):  # pylint: disable=invalid-name
        # type: (int) -> None
        self._id = new

    @property
    def math(self):
        """A convenience property for fetching the functions available in
        `coach_os.math_utils <coach_os.html#module-coach_os.math_utils>`_.

        For example, you can do:

        .. code-block:: python

           pos, theta = robot.get_pose_blocking()
           robot.logger.info(
                'My position is: %s' % (robot.math.clamp_angle(theta)))

        Returns:
            module: All functions in math_utils.
        """
        return math_utils

    @property
    def units(self):
        """
        A convenience property for fetching the functions available in
        `coach_os.units <coach_os.html#module-coach_os.units>`_.

        Example:

        .. code-block:: python

           robot.units.convert_distance(1, 'm', 'cm')  # Returns 100

        Returns:
            module: All functions in units.
        """
        return units

    @property
    def coordinates(self):
        """A convenience property for fetching the functions available in
        `coach_os.coordinates <coach_os.html#module-coach-os.coordinates>`_.

        Example:

        .. code-block:: python

           robot.coordinates.bot_in_bounds(np.ndarray([[0, 0], [0, 0]]))

        Returns:
            module: All functions in coordinates.
        """
        return coordinates

    @property
    def configuration(self):
        """
        A convenience property for fetching the functions available in
        `coach_os.configuration
        <coach_os.html#module-coach_os.configuration>`_.

        Example:

        .. code-block:: python

           robot.configuration.get_is_debug()

        Returns:
            module: All functions in configuration.
        """
        return configuration

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
        pass

    def set_vel(self, left, right):
        # type: (int|float, int|float) -> None
        """
        Sets the speed for the left and right wheel in percentage values.

        Parameters:
            left (int): The left motor speed (-100 - 100)
            right (int): The right motor speed (-100 - 100)
        """

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
        # type: () -> tuple[float, float, float] | None
        """
        This function retrieves the pose of the robot, if it can. If it can't
        it returns None.

        Returns:
            tuple[float, float, float] | None: The global pose as a tuple (x,
            y, theta) if new data available since last invokation, None
            otherwise.
        """

    def get_pose_blocking(self, delay_millis=200.0):
        # type: (float) -> tuple[Vec2, float]
        """
        Returns the pose of the bot. This function waits until data is
        available.

        Warning:
            This function may enter a state where no data can be retrieved.
            This can occur when the robot exits the bounds of the playpen. In
            that case, this function will simply **block execution**.

        Returns:
            tuple[Vec2, float]: The pos_x, pos_y, theta of the robot.
        """

    def delay(self, millis=200):
        # type: (float) -> None
        """Waits some miliseconds (default 200).

        Parameters:
            millis: The amount of time to wait.

        Todo:
            Handle better the complicated exception that is raised.
        """
