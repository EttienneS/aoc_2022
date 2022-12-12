# aoc_2022

Solutions to the [2022 Advent Of Code](https://adventofcode.com/2022) written in Rust.

## Notes

Stuff that I should probably have done better for each day. As I am doing this to actually learn Rust it seems like a good idea to write down my thoughts on each day so that I can learn from them if I ever revisit this.

### Day 1

Day is not too bad for a very simple problem, I could probably have used map() to get the solution without using loops to make the code a bit shorter. This day I made the input parser that is used for other steps.

```md
Part 1 solution: 70374
Part 2 solution: 204610
```

### Day 2

This one is again fine but it can be done very simply by just having each possible move/outcome combo in a dictionary. But I like my version better.

```md
Part 1 solution: 13052
Part 2 solution: 13693
```

### Day 3

This one again I could have use map() and filter() to make my life easier, here I realized parsing was going to be a problem and I should have enhanced the parser to make my life easier.

```md
Part 1 solution: 7811
Part 2 solution: 2639
```

### Day 4

Finally started getting a bit better with the collection methods to help with the parsing. The overlapping pairs check took way longer than it should have.

```md
Part 1 solution: 413
Part 2 solution: 806
```

### Day 5

Another parsing nightmare, the code to read the actual commands still haunts me... but I managed to get it done, the VecDeque struct is very useful for these types of things as it can push/pop on both ends.

```md
Part 1 solution: TBVFVDZPN
Part 2 solution: VLCWHTDSZ
```

### Day 6

The simplest one so far, my solution is not perfect I should have used something else to get the sliding window rather than nested loops but it works quick. Looking at solutions for this one I notices a lot of folks using itertools so I added it to the solution but have not used it at this point.

```md
Part 1 solution: 1920
Part 2 solution: 2334
```

### Day 7

The hardest one, I spent so much time trying to figure out a way to make a tree work the way I would make it in something like c# with no luck. I finally caved and looked at other solutions and found the indextree crate to help. But even with that I had to get a lot of help from other's solutions.

```md
Part 1 solution: 1232307
Part 2 solution: 7268994
```

### Day 8

Back to form at least, I managed to solve this one pretty easily but in retrospect I should have used a 2D grid instead of a list of objects with coords. The lookups for each row/column could be muuuuch faster.

```md
Part 1 solution: 1785
Part 2 solution: 345168
```

### Day 9

Started typing up these reports so I retroactively went back and did the other days.

```md
Part 1 solution: XXX
Part 2 solution: XXX
```

Day 9 is a bit of a bust, I made two solutions that are close but not working exactly with the example set.

Shelving this one for now to be revisited later.

### Day 10

This one is fairly easy for the first part but the second part hard me stumped, the prompt was a bit confusing/it was very late. But some sleep helped me solve it pretty quickly the next morning.

I have a weird "off-by-one" issue with the output but that is easy enough to eyeball fix.

Flagging this one to revisit after this is all done to fix that.

### Day X

```md
Part 1 solution: XXX
Part 2 solution: XXX
```
