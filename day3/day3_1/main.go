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
	row := 0
	trees := 0

	for scanner.Scan() {
		if row > 0 {
			line := scanner.Text()
			index := (row * 3) % 31

			if line[index] == '#' {
				trees++
			}
		}

		row++
	}

	fmt.Println(trees)
}
