package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type Play struct {
	player   byte
	opponent byte
}

var WINS_AGAINST = map[byte]byte{
	'A': 'C', 'B': 'A', 'C': 'B',
}

var LOSES_AGAINST = map[byte]byte{
	'C': 'A', 'A': 'B', 'B': 'C',
}

var PLAY_SCORES = map[byte]uint8{
	'A': 1, 'B': 2, 'C': 3,
}

func get_player_action(opponent_action byte, player_strategy byte) byte {
	if player_strategy == 'X' {
		// lose
		return WINS_AGAINST[opponent_action]
	}
	if player_strategy == 'Z' {
		// win
		return LOSES_AGAINST[opponent_action]
	}
	// draw
	return opponent_action
}

func get_play(line string) Play {
	tokens := strings.Split(line, " ")
	opponent_action := tokens[0][0]
	player_strategy := tokens[1][0]
	player_action := get_player_action(opponent_action, player_strategy)
	return Play{player: player_action, opponent: opponent_action}
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
		play := get_play(line)
		player_score, opponent_score := get_play_outcome(play)
		total_player_score += player_score
		total_opponent_score += opponent_score
	}

	fmt.Println(total_player_score, total_opponent_score)

}
