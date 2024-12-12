day="$1"

. .aoc_session_token

if [ -z "$AOC_SESSION_TOKEN" ]
then
    echo "AOC_SESSION_TOKEN is not set"
    exit 1
fi

if [ -z "$day" ]
then
    echo "Usage: $0 <day number>"
    exit 1
fi

mkdir -p inputs
curl \
    --fail \
    --silent \
    --show-error \
    --cookie "$AOC_SESSION_TOKEN" \
    "https://adventofcode.com/2024/day/$day/input" \
    > "inputs/day$day.txt"
