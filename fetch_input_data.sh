#!/bin/zsh

DAY=$(date|awk '{ printf("%02d\n", $3) }')
YEAR=$(date|awk '{ print($6) }')

WORKSPACE_DIR=~/projects/advent-of-code-$YEAR/
PROJECT_DIR=day-$DAY

mkdir -p $WORKSPACE_DIR
cd $WORKSPACE_DIR
if [[ -z $(ls|grep "$PROJECT_DIR") ]]; then
    cargo init $PROJECT_DIR
fi
cd $PROJECT_DIR
cp ~/Library/Application\ Support/Firefox/Profiles/*.default-release/cookies.sqlite ./cookies_copy.sqlite

SESSION_COOKIE=$(sqlite3 ./cookies_copy.sqlite "select name,value from moz_cookies where host = '.adventofcode.com'"\
    | sed 's/|/=/')
rm ./cookies_copy.sqlite*

curl -b $SESSION_COOKIE https://adventofcode.com/$YEAR/day/$(echo $DAY| bc)/input > input.txt
