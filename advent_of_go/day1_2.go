package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
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

	var carbs []uint64
	var current_sum uint64 = 0
	for fileScanner.Scan() {
		line := fileScanner.Text()
		if len(line) == 0 {
			carbs = append(carbs, current_sum)
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

	sort.Slice(carbs, func(i, j int) bool {
		return carbs[i] > carbs[j]
	})

	biggest_three := carbs[0:3]
	var total uint64 = 0
	for i := 0; i < len(biggest_three); i++ {
		total += biggest_three[i]
	}

	fmt.Println(total)

}
