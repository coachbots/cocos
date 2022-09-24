from abc import ABC, abstractmethod
from typing import Any, Dict, Tuple

PositionT = Tuple[float, float, float]
LedColorT = Tuple[float, float, float]
MotorPowerT = Tuple[float, float]
CoachbotConfigT = Dict[str, Any]


class BaseCoachbot(ABC):
    @abstractmethod
    def on_configure(self) -> CoachbotConfigT:
        """Returns the desired Coachbot Configuration. This configuration must
        specify all the desired configuration fields.

        You should override this method to expose your own behavior.

        Valid fields are:
        * comm_range: The communication range in meters.
                      Setting this feature to any value other than 0 causes
                      messages of a length greater than said value to be
                      ignored. Set to 0 to disable. Default: 0
        * script_name: The name of the script. Optional.
        * script_author: The name of the author of this script. Optional.
        * script_version: The version of the current script in symver form.
                          Optional

        Example usage:

        .. code-block:: python

           def on_configure(self) -> CoachbotConfigT:
               return {
                   'comm_range': 3,
                   'script_name': 'Macondo',
                   'script_author': 'Remedios the Beauty',
                   'script_version': '0.3.1'
               }
        """
        raise NotImplementedError()

    @property
    def identifier(self) -> int:
        """Returns the identification number of self."""

        raise NotImplementedError()

    def get_position(self) -> PositionT:
        """Returns current coachbot position."""

        raise NotImplementedError()

    def set_led(self, led_color: LedColorT) -> None:
        """Sets the LED color.

        Parameters:
            led_color (LedColorT): A 3-tuple of float values between 0 and 1
            for RGB color intensities.
        """
        for col in led_color:
            if not 0 <= col <= 1:
                raise ValueError('The LED color values must be 0 ≤ x ≤ 1.')

        raise NotImplementedError()

    def set_motor_pows(self, motor_powers: MotorPowerT) -> None:
        """Sets the motor powers.

        Parameters:
            motor_powers (MotorPowerT): A 2-tuple of float values between -1
                                        and 1.
        """
        for col in motor_powers:
            if not -1 <= col <= 1:
                raise ValueError('The Motor power values must be 0 ≤ x ≤ 1.')

        raise NotImplementedError()


class TickCoachbot(BaseCoachbot):
    pass


class ReactiveCoachbot(BaseCoachbot):
    pass
