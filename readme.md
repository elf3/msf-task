```
$ cargo run -- -f test-journal.json add "buy milk"

$ cargo run -- -f test-journal.json add "take the dog for a walk"

$ cargo run -- -f test-journal.json add "water the plants"

$ cargo run -- -f test-journal.json list
1 : buy milk                                           [2022-02-15 12:12]
2 : take the dog for a walk                            [2022-02-15 12:12]
3 : water the plants                                   [2022-02-15 12:13]

$ cargo run -- -f test-journal.json done 2

$ cargo run -- -f test-journal.json list
1 : buy milk                                           [2022-02-15 12:12]
2 : water the plants                                   [2022-02-15 12:13]


cargo run -- -f missing-journal done 2
```