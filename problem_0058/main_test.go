package main

import (
  "testing"
  "github.com/stretchr/testify/assert"
)

func TestIsPrime(t *testing.T) {
    assert.Equal(t, isPrime(1), false)
    assert.Equal(t, isPrime(3), true)
    assert.Equal(t, isPrime(2), false)
    assert.Equal(t, isPrime(31), true)
    assert.Equal(t, isPrime(34), false)
}

func TestProblem58(t *testing.T) {
    assert.Equal(t, diagonal(4), 0.6153846153846154)
}
