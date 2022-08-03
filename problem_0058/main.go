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

func problem58() int {
    var start int = 1
    var total int = 1
    var primes int = 0
    var factor int = 2

    for {
        for k := 0; k < 4; k++ {
            start += factor
            total++

            if isPrime(start) {
                primes++
            }
        }

        factor += 2

        if (float64(primes) / float64(total)) < 0.10 {
            break
        }
    }

    return factor - 1
}

func main() {
    fmt.Println("Hello world.")
}
