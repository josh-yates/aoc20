package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	file, err := os.Open("/home/josh/Downloads/day3")

	if err != nil {
		fmt.Println("Could not load file")
		return
	}

	defer file.Close()
	scanner := bufio.NewScanner(file)
	var lines []string

	for scanner.Scan() {
		lines = append(lines, scanner.Text())
	}

	result := countTrees(lines, 1, 1) *
		countTrees(lines, 3, 1) *
		countTrees(lines, 5, 1) *
		countTrees(lines, 7, 1) *
		countTrees(lines, 1, 2)

	fmt.Println(result)
}

func countTrees(lines []string, across int, down int) int {
	trees := 0
	linesChecked := 0

	for row := 0; row < len(lines); row += down {
		line := lines[row]
		index := (linesChecked * across) % len(line)

		if line[index] == '#' {
			trees++
		}

		linesChecked++
	}

	return trees
}
