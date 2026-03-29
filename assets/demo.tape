Output assets/demo.gif

Set FontSize 16
Set Width 1000
Set Height 500
Set Theme "Catppuccin Mocha"
Set Shell "bash"

# Demo 1: Alphabetical real-time sort
Type "# 1. Alphabetical real-time sort"
Enter
Sleep 0.5s
Type "(for w in zebra apple mango kiwi banana orange grape cherry; do echo $w; sleep 0.4; done) | rtsort"
Sleep 0.5s
Enter
Sleep 6s

Type "clear"
Enter
Sleep 0.5s

# Demo 2: Human-readable numeric sort, top 3
Type "# 2. Human-readable numeric sort, top 3 largest  (-h -r --top 3)"
Enter
Sleep 0.5s
Type "(for s in 2G 500M 1K 3T 100M 50G 200K; do echo $s; sleep 0.4; done) | rtsort -h -r --top 3"
Sleep 0.5s
Enter
Sleep 4s

Type "clear"
Enter
Sleep 0.5s

# Demo 3: Sort by field with key and separator
Type "# 3. Sort by field with key and separator  (-k 2 -t ':')"
Enter
Sleep 0.5s
Type "(for e in alice:30 bob:5 charlie:200 dave:50 eve:1; do echo $e; sleep 0.4; done) | rtsort -k 2 -t : -n"
Sleep 0.5s
Enter
Sleep 3s
