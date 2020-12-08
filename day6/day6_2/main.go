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

	var seen = make(map[rune]int)
	countSum := 0
	groupMemberCount := 0

	for scanner.Scan() {
		line := scanner.Text()

		if line == "" {
			countSum += processBatch(seen, groupMemberCount)
			seen = make(map[rune]int)
			groupMemberCount = 0
			continue
		}

		groupMemberCount++

		for _, char := range line {
			charCount := seen[char]
			seen[char] = charCount + 1
		}
	}

	countSum += processBatch(seen, groupMemberCount)

	fmt.Println(countSum)
}

func processBatch(batch map[rune]int, lineCount int) int {
	sum := 0
	for _, value := range batch {
		if value == lineCount {
			sum++
		}
	}

	return sum
}
