day="$1"

if [ -z "$day" ]
then
    echo "Usage: $0 <day number>"
    exit 1
fi
mkdir -p inputs
curl \
    --cookie "$AOC_SESSION_TOKEN" \
    "https://adventofcode.com/2024/day/$day/input" \
    > "inputs/day$day.txt"
