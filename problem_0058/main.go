package main

import (
  "fmt"
  "math"
)

func isPrime(num int) bool {
    if num < 2 {
      return false
    }

    var isPrime bool = true
    var end int = int(math.Floor(math.Sqrt(float64(num))))

    for i := 2; i <= end + 1; i++ {
        if num % i == 0 {
            isPrime = false
            break
        }
    }

    return isPrime
}

func diagonal(size int) float64 {
    var start int = 1
    var total int = 1;
    var primes int = 0;

    for factor := 1; factor < size; factor++ {
        for k := 0; k < 4; k++ {
            start += (factor * 2)
            total++

            if isPrime(start) {
                primes++
            }
        }
    }

    return float64(primes) / float64(total)
}

func main() {
    fmt.Println("Hello world.")
}
