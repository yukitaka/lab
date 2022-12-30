package main

import (
	"fmt"
	"math/rand"
	"time"
)

func main() {
	var size, seed, gen int
	fmt.Scan(&size, &seed, &gen)
	rand.Seed(int64(seed))
	cells := make([][]int, size)
	for y := 0; y < size; y++ {
		cells[y] = make([]int, size)
		for x := 0; x < size; x++ {
			cells[y][x] = rand.Intn(2)
		}
	}

	for i := 0; i < gen; i++ {
		sum := evolute(cells)
		showHeader(i+1, sum)
		showCells(cells)
		time.Sleep(500 * time.Millisecond)
	}
}

func showHeader(gen, alive int) {
	fmt.Print("\033[H\033[2J")
	fmt.Printf("Generation #%d\n", gen)
	fmt.Printf("Alive: %d\n", alive)
}

func showCells(cells [][]int) {
	for _, row := range cells {
		for _, col := range row {
			if col == 1 {
				fmt.Print("O")
			} else {
				fmt.Print(" ")
			}
		}
		fmt.Println()
	}
}

func evolute(cells [][]int) int {
	size := len(cells)
	cnt := make([][]int, size)
	sum := 0
	for y, row := range cells {
		cnt[y] = make([]int, size)
		for x := range row {
			cnt[y][x] = around(cells, y, x)
		}
	}
	for y, row := range cells {
		for x, col := range row {
			if col == 1 {
				if cnt[y][x] < 2 || cnt[y][x] > 3 {
					cells[y][x] = 0
				} else {
					sum++
				}
			} else if cnt[y][x] == 3 {
				cells[y][x] = 1
				sum++
			}
		}
	}

	return sum
}

func around(cells [][]int, y, x int) int {
	size := len(cells)
	left := pos(x, -1, size-1, size-1)
	right := pos(x, 1, size-1, 0)
	up := pos(y, -1, size-1, size-1)
	down := pos(y, 1, size-1, 0)

	around := cells[up][left] + cells[up][x] + cells[up][right]
	around += cells[y][left] + cells[y][right]
	around += cells[down][left] + cells[down][x] + cells[down][right]

	return around
}

func pos(current, direction, size, fallback int) int {
	idx := current + direction
	if idx < 0 || idx > size {
		return fallback
	}

	return idx
}
