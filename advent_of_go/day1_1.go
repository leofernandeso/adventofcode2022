package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {

	filepath, err := os.Open(os.Args[1])

	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	fileScanner := bufio.NewScanner(filepath)
	fileScanner.Split(bufio.ScanLines)

	var current_sum uint64 = 0
	var biggest_sum uint64 = 0
	for fileScanner.Scan() {
		line := fileScanner.Text()
		if len(line) == 0 {
			if current_sum > biggest_sum {
				biggest_sum = current_sum
			}
			current_sum = 0
			continue
		}
		line_uint, err := strconv.ParseUint(line, 10, 64)

		if err != nil {
			fmt.Println("Error parsing line.")
			os.Exit(1)
		}

		current_sum += line_uint
	}

	fmt.Println(biggest_sum)

}
