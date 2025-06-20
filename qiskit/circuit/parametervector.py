# This code is part of Qiskit.
#
# (C) Copyright IBM 2017, 2019.
#
# This code is licensed under the Apache License, Version 2.0. You may
# obtain a copy of this license in the LICENSE.txt file in the root directory
# of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.
#
# Any modifications or derivative works of this code must retain this
# copyright notice, and modified files need to carry a notice indicating
# that they have been altered from the originals.

"""Parameter Vector Class to simplify management of parameter lists."""

from uuid import uuid4, UUID

from .parameter import Parameter, ParameterExpression

import qiskit._accelerate.circuit

ParameterExpressionBase = qiskit._accelerate.circuit.ParameterExpression


class ParameterVectorElement(Parameter):
    """An element of a :class:`ParameterVector`.

    .. note::
        There is very little reason to ever construct this class directly.  Objects of this type are
        automatically constructed efficiently as part of creating a :class:`ParameterVector`.
    """

    ___slots__ = ("_vector", "_index")

    def __new__(cls, vector=None, index=None, uuid=None):

        if uuid != None:
            uuid = int(uuid)
        elif vector == None:
            return super().__new__(cls, None, None)

        self = super().__new__(cls, f"{vector.name}[{index}]", uuid=uuid)
        self._vector = vector
        self._index = index
        return self

    #        return super(Parameter, self).__new__(self, None, ParameterExpressionBase.VectorElement(vector.name, index, uuid=uuid))

    @property
    def index(self):
        """Get the index of this element in the parent vector."""
        return self._index

    @property
    def vector(self):
        """Get the parent vector instance."""
        return self._vector

    def __getstate__(self):
        return (super().__getstate__(), self._vector, self._index)

    def __setstate__(self, state):
        (super_state, vector, index) = state
        super().__setstate__(super_state)
        self._vector = vector
        self._index = index


class ParameterVector:
    """A container of many related :class:`Parameter` objects.

    This class is faster to construct than constructing many :class:`Parameter` objects
    individually, and the individual names of the parameters will all share a common stem (the name
    of the vector).  For a vector called ``v`` with length 3, the individual elements will have
    names ``v[0]``, ``v[1]`` and ``v[2]``.

    The elements of a vector are sorted by the name of the vector, then the numeric value of their
    index.

    This class fulfill the :class:`collections.abc.Sequence` interface.
    """

    __slots__ = ("_name", "_params", "_root_uuid")

    def __init__(self, name, length=0):
        self._name = name
        self._root_uuid = uuid4()
        root_uuid_int = self._root_uuid.int
        self._params = [
            ParameterVectorElement(self, i, UUID(int=root_uuid_int + i)) for i in range(length)
        ]

    @property
    def name(self):
        """The name of the :class:`ParameterVector`."""
        return self._name

    @property
    def params(self):
        """A list of the contained :class:`ParameterVectorElement` instances.

        It is not safe to mutate this list."""
        return self._params

    def index(self, value):
        """Find the index of a :class:`ParameterVectorElement` within the list.

        It is typically much faster to use the :attr:`ParameterVectorElement.index` property."""
        return self._params.index(value)

    def __getitem__(self, key):
        return self.params[key]

    def __iter__(self):
        return iter(self.params)

    def __len__(self):
        return len(self._params)

    def __str__(self):
        return f"{self.name}, {[str(item) for item in self.params]}"

    def __repr__(self):
        return f"{self.__class__.__name__}(name={repr(self.name)}, length={len(self)})"

    def resize(self, length):
        """Resize the parameter vector.  If necessary, new elements are generated.

        Note that the UUID of each :class:`.Parameter` element will be generated
        deterministically given the root UUID of the ``ParameterVector`` and the index
        of the element.  In particular, if a ``ParameterVector`` is resized to
        be smaller and then later resized to be larger, the UUID of the later
        generated element at a given index will be the same as the UUID of the
        previous element at that index.
        This is to ensure that the parameter instances do not change.

        >>> from qiskit.circuit import ParameterVector
        >>> pv = ParameterVector("theta", 20)
        >>> elt_19 = pv[19]
        >>> rv.resize(10)
        >>> rv.resize(20)
        >>> pv[19] == elt_19
        True
        """
        if length > len(self._params):
            root_uuid_int = self._root_uuid.int
            self._params.extend(
                [
                    ParameterVectorElement(self, i, UUID(int=root_uuid_int + i))
                    for i in range(len(self._params), length)
                ]
            )
        else:
            del self._params[length:]
