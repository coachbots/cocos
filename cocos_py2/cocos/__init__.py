#!/usr/bin/env python2

from __future__ import absolute_import, print_function
from abc import abstractmethod, ABCMeta
#from typing import ByteString, Tuple, Dict, Union
import json
import zmq
from enum import IntEnum
from timeit import default_timer


MESSAGE_ENCODING = 'ascii'


# For whatever reason I could not get IntEnum to behave.
IPC_MESSAGE_TYPES = {
    'LED': 0,
    'VEL': 1
}


class IPCMessage(object):
    """A message object that can be sent over the network."""
    __metaclass__ = ABCMeta
    __TYPE__ = -1

    @abstractmethod
    def serialize(self):
        """Serializes the message data."""
        # type: () -> bytes
        pass

    def serialize_total(self):
        # type: () -> bytes
        return json.dumps({
            'request_type': self.__class__.__TYPE__,
            'body': self.serialize()
        }).encode(MESSAGE_ENCODING)


class IPCSendLedMessage(IPCMessage):
    __TYPE__ = IPC_MESSAGE_TYPES['LED']
    _MINIMUM_COLOR = 0
    _MAXIMUM_COLOR = 100

    def __init__(self, rgb):
        # type: (Tuple[int, int, int]) -> None
        for val in rgb:
            if val < self.__class__._MINIMUM_COLOR \
                    or val > self.__class__._MAXIMUM_COLOR:
                raise ValueError('Colors must be 0 <= color <= 100')
        self._rgb = rgb

    def serialize(self):
        # type: () -> bytes
        return json.dumps({
            'r': self._rgb[0],
            'g': self._rgb[1],
            'b': self._rgb[2]
        }).encode(MESSAGE_ENCODING)


class IPCSendVelocityMessage(IPCMessage):
    __TYPE__ = IPC_MESSAGE_TYPES['VEL']
    _MINIMUM_VEL = 0
    _MAXIMUM_VEL = 100

    def __init__(self, velocities):
        # type: (Tuple[int, int]) -> None
        for val in velocities:
            if val < self.__class__._MINIMUM_VEL \
                    or val > self.__class__._MAXIMUM_VEL:
                raise ValueError('Velocities must be 0 <= color <= 100')
        self._vels = velocities

    def serialize(self):
        # type: () -> bytes
        return json.dumps({
            'l': self._vels[0],
            'r': self._vels[1]
        }).encode(MESSAGE_ENCODING)


class IPCStatus(IntEnum):
    SUCCESS = 0
    INVALID_ENCODING = 1,
    INVALID_REQUEST_HEAD = 2,
    INVALID_REQUEST_BODY = 3,
    INVALID_REQUEST_ARGS = 4


class IPCResponse(object):
    def __init__(self, as_bytes):
        # type: (bytes) -> None
        self._data = as_bytes
        self.__deserialized = None

    @property
    def deserialized(self):
        # type: () -> Dict[str, Union[bytes, int, str]]
        if self.__deserialized is not None:
            return self.__deserialized

        result = json.loads(self._data)
        if result.get('status') is None:
            raise ValueError('The Response from Cocos does not contain a '
                             'status')
        result['status'] = IPCStatus(result['status'])
        self.__deserialized = result
        return self.__deserialized


class IPCResponseError(IPCResponse):
    def __init__(self, as_bytes):
        # type: (bytes) -> None
        IPCResponse.__init__(self, as_bytes)
        self.__error_deserialized = None

    @property
    def deserialized(self):
        # type: () -> Dict[str, Union[bytes, int, str]]
        if self.__error_deserialized is not None:
            return self.__error_deserialized

        parent_result = super(IPCResponseError, self).deserialized
        assert isinstance(parent_result['body'], bytes)
        parent_result['body'] = parent_result['body'].decode('ascii')
        self.__error_deserialized = parent_result
        return self.__error_deserialized


class IPCError(Exception):
    pass

class IPCInvalidResponse(IPCError):
    pass


class IPCMessager(object):
    __RESPONSE_TIMEOUT = 3  # The maximum acceptable timeout in ms before a
                            # no-response is asserted.

    def __init__(self, tx_addr):
        # type: (str) -> None
        self._tx_addr = tx_addr
        self._tx_ctx = zmq.Context()
        self._tx_sock = self._tx_ctx.socket(zmq.REQ)
        self._time_since_last_send = default_timer()

    def begin(self):
        # type: () -> None
        self._tx_sock.connect(self._tx_addr)

    def _restart(self):
        # type: () -> None
        self._tx_sock.setsockopt(zmq.LINGER, 0)
        self._tx_sock.close()
        self._tx_sock = self._tx_ctx.socket(zmq.REQ)
        self._tx_sock.connect(self._tx_addr)

    def tx(self, message):
        # type: (IPCMessage) -> None
        try:
            # Send a message
            self._tx_sock.send(message.serialize_total())

            # Wait for response.
            if not self._tx_sock.poll(self.__class__.__RESPONSE_TIMEOUT):
                raise zmq.ZMQError(zmq.EAGAIN,
                                   'Timed out waiting for response')
            msg = self._tx_sock.recv()
            response = IPCResponse(bytes(msg))

            # Ensure that the status is successful.
            if response.deserialized['status'] != IPCStatus.SUCCESS:
                response = IPCResponseError(bytes(msg))
                raise IPCInvalidResponse(response.deserialized['body'])
        except (zmq.ZMQError, AssertionError) as err:
            # TODO: Warn user
            # Error ocurred, so the best we can do is recreate the socket in
            # hopes it will work this time around.
            self._restart()


class CocosCommunicator(object):
    def __init__(self, tx_addr):
        # type: (str) -> None
        self.messages = {
            IPCSendLedMessage.__TYPE__: None,
            IPCSendVelocityMessage.__TYPE__: None
        }
        self._messager = IPCMessager(tx_addr)

    def begin(self):
        # type: () -> None
        self._messager.begin()

    def send_led(self, rgb):
        # type: (Tuple[int, int, int]) -> None
        """Sends the LED update message.

        Raises:
            ValueError: Upon the RGB values being invalid.
        """
        message = IPCSendLedMessage(rgb)
        self._messager.tx(message)

    def send_vel(self, velocities):
        # type: (Tuple[int, int]) -> None
        """Sends the velocity update message.

        Raises:
            ValueError: Upon the velocity values being invalid.
        """
        message = IPCSendVelocityMessage(velocities)
        self._messager.tx(message)
