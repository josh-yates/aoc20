package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("/home/josh/Downloads/day6")

	if err != nil {
		fmt.Println("Could not load file")
		return
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)

	var seen = make(map[rune]bool)
	countSum := 0

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			countSum += len(seen)
			seen = make(map[rune]bool)
			continue
		}

		for _, char := range line {
			seen[char] = false
		}
	}

	countSum += len(seen)

	fmt.Println(countSum)
}
