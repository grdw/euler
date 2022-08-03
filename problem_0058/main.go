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

func main() {
    fmt.Println("Hello world.")
}
