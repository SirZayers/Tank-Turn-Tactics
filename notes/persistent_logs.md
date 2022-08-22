# two files: definitive + future

1. definitive_board.txt

... actions

2. future_board.txt (+ lock: only one person may write this file)

3. rename future_board.txt -> definitive_board.txt

4. (remove lock)

... actions

2.-4. repeat

# one file, only append

# one board-config + one append-only log

Board config (Board struct):

- id

- size

- player

- ...

Append only log:

- log-id

- player

- options
