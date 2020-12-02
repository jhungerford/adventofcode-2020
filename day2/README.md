# Day 2
[Day 2](https://adventofcode.com/2020/day/2) involves counting the number of valid passwords in a file.  Policies consist of a range and a letter, and we need to check whether the count of the letter in the password fits in the range.

Part 2 changes the policy, but still asks for a count of valid passwords.

I parsed each line into a `PasswordPolicy` struct, and implemented the rules as methods.  Converting lines into structs is easy with the `FromStr` trait, and the password lines were complicated enough that I used a regex to parse them.
