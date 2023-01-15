package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
	// "strconv"
)

type Play struct {
	player   byte
	opponent byte
}

var PLAY_SCORES = map[byte]uint8{
	'A': 1, 'B': 2, 'C': 3,
	'X': 1, 'Y': 2, 'Z': 3,
}

var WINS_AGAINST = map[byte]byte{
	'A': 'Z', 'B': 'X', 'C': 'Y',
	'X': 'C', 'Y': 'A', 'Z': 'B',
}

func parse_line(line string) Play {
	tokens := strings.Split(line, " ")
	player := tokens[1]
	opponent := tokens[0]
	return Play{player: player[0], opponent: opponent[0]}
}

func get_play_outcome(play Play) (uint32, uint32) {

	// Initial score for each player
	player_score := uint32(PLAY_SCORES[play.player])
	opponent_score := uint32(PLAY_SCORES[play.opponent])

	// Determining who wins the match
	if WINS_AGAINST[play.player] == play.opponent {
		player_score += 6
	} else if WINS_AGAINST[play.opponent] == play.player {
		opponent_score += 6
	} else {
		player_score += 3
		opponent_score += 3
	}

	return player_score, opponent_score

}

func main() {

	if len(os.Args) == 1 {
		fmt.Println("Provide a valid input path!")
	}

	filepath, err := os.Open(os.Args[1])

	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	fileScanner := bufio.NewScanner(filepath)
	fileScanner.Split(bufio.ScanLines)

	var total_player_score uint32 = 0
	var total_opponent_score uint32 = 0
	for fileScanner.Scan() {
		line := fileScanner.Text()
		play := parse_line(line)
		player_score, opponent_score := get_play_outcome(play)
		total_player_score += player_score
		total_opponent_score += opponent_score
	}

	fmt.Println(total_player_score, total_opponent_score)

}
