package main

import (
  "fmt"
  "math"
)

func isPrime(num int) bool {
    if num < 2 {
      return false
    }

    var is_prime bool = true
    var end int = int(math.Floor(math.Sqrt(float64(num))))

    for i := 2; i <= end + 1; i++ {
        if num % i == 0 {
            is_prime = false
            break
        }
    }

    return is_prime
}

func main() {
    fmt.Println("Hello world.")
}
