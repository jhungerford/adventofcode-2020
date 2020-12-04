# Day 4
[Day 4](https://adventofcode.com/2020/day/4) is about passport validation - checking whether they have required fields and whether the fields are valid.  The passport data isn't strictly formatted - fields can appear on any number of lines and passports are separated by newlines.

I built up a buffer of fields which I flushed into a passport to parse the passports, and relied heavily on regexes to validate  values. 