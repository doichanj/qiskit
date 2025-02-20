# This code is part of Qiskit.
#
# (C) Copyright IBM 2017, 2024.
#
# This code is licensed under the Apache License, Version 2.0. You may
# obtain a copy of this license in the LICENSE.txt file in the root directory
# of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.
#
# Any modifications or derivative works of this code must retain this
# copyright notice, and modified files need to carry a notice indicating
# that they have been altered from the originals.

"""Tests for SparsePauliOp class."""

import itertools as it
import unittest
from test import QiskitTestCase, combine

import numpy as np
import rustworkx as rx
import scipy.sparse
import ddt

import inspect

from qiskit import QiskitError
from qiskit.circuit import Parameter, ParameterExpression, ParameterVector
from qiskit.circuit.library import efficient_su2
from qiskit.circuit.parametertable import ParameterView
from qiskit.compiler.transpiler import transpile
from qiskit.primitives import BackendEstimator
from qiskit.providers.fake_provider import GenericBackendV2
from qiskit.quantum_info import SparseObservable
from qiskit.quantum_info.operators import (
    Operator,
    Pauli,
    PauliList,
    SparsePauliOp,
)
from qiskit.utils import optionals


def pauli_mat(label):
    """Return Pauli matrix from a Pauli label"""
    mat = np.eye(1, dtype=complex)
    for i in label:
        if i == "I":
            mat = np.kron(mat, np.eye(2, dtype=complex))
        elif i == "X":
            mat = np.kron(mat, np.array([[0, 1], [1, 0]], dtype=complex))
        elif i == "Y":
            mat = np.kron(mat, np.array([[0, -1j], [1j, 0]], dtype=complex))
        elif i == "Z":
            mat = np.kron(mat, np.array([[1, 0], [0, -1]], dtype=complex))
        else:
            raise QiskitError(f"Invalid Pauli string {i}")
    return mat


def bind_parameters_to_one(array):
    """Bind parameters to one. The purpose of using this method is to bind some value and
    use ``assert_allclose``, since it is impossible to verify equivalence in the case of
    numerical errors with parameters existing.
    """

    def bind_one(a):
        parameters = a.parameters
        return complex(a.bind(dict(zip(parameters, [1] * len(parameters)))))

    return np.vectorize(bind_one, otypes=[complex])(array)


@ddt.ddt
class TestSparsePauliOpMethods(QiskitTestCase):
    """Tests for SparsePauliOp operator methods."""

    RNG = np.random.default_rng(1994)

    def setUp(self):
        super().setUp()

        self.parameter_names = (f"param_{x}" for x in it.count())

    def random_spp_op(self, num_qubits, num_terms, use_parameters=False):
        """Generate a pseudo-random SparsePauliOp"""
        if use_parameters:
            coeffs = np.array(ParameterVector(next(self.parameter_names), num_terms))
        else:
            coeffs = self.RNG.uniform(-1, 1, size=num_terms) + 1j * self.RNG.uniform(
                -1, 1, size=num_terms
            )
        labels = [
            "".join(self.RNG.choice(["I", "X", "Y", "Z"], size=num_qubits))
            for _ in range(num_terms)
        ]
        return SparsePauliOp(labels, coeffs)

    @combine(num_qubits=[1, 2, 3, 4], num_ops=[1, 2, 3, 4], param=[None, "a"])
    def test_sum(self, num_qubits, num_ops, param):
        """Test sum method for {num_qubits} qubits with {num_ops} operators."""
        print("  >>>>>>  test num_qubits = ", num_qubits,num_ops,param, inspect.currentframe().f_code.co_name)
        ops = [
            self.random_spp_op(
                num_qubits, 2**num_qubits, param if param is None else f"{param}_{i}"
            )
            for i in range(num_ops)
        ]
        print("  =========  random_spp =====================", num_qubits,num_ops,param)
        sum_op = SparsePauliOp.sum(ops)
        print("  =========  sum_op =====================", num_qubits,num_ops,param)
        value = sum_op.to_matrix()
        print("  =========  value =====================", num_qubits,num_ops,param)
        target_operator = ops[0].to_matrix()
        for op in ops[1:]:
            sum(op, target_operator)
#        target_operator = sum((op.to_matrix() for op in ops[1:]), ops[0].to_matrix())
#        print("  =========  target_operator =====================", num_qubits,num_ops,param)
#        if param is not None:
#            value = bind_parameters_to_one(value)
#            target_operator = bind_parameters_to_one(target_operator)
#        print("  =========  Bound =====================", num_qubits,num_ops,param)
#        np.testing.assert_allclose(value, target_operator, atol=1e-8)
#        target_spp_op = sum((op for op in ops[1:]), ops[0])
#        self.assertEqual(sum_op, target_spp_op)
#        print("  =========  EQ 1 =====================", num_qubits,num_ops,param)
#        np.testing.assert_array_equal(sum_op.paulis.phase, np.zeros(sum_op.size))
#        print("  =========  EQ 2 =====================", num_qubits,num_ops,param)

if __name__ == "__main__":
    unittest.main()