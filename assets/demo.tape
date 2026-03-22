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

# Demo 2: Numeric sort
Type "# 2. Numeric sort  (-n)"
Enter
Sleep 0.5s
Type "(for n in 42 7 100 3 55 18 1 999; do echo $n; sleep 0.4; done) | rtsort -n"
Sleep 0.5s
Enter
Sleep 4s

Type "clear"
Enter
Sleep 0.5s

# Demo 3: Human-readable numeric sort, top 3
Type "# 3. Human-readable size sort, top 3  (-h -t 3)"
Enter
Sleep 0.5s
Type "(for s in 2G 500M 1K 3T 100M 50G 200K; do echo $s; sleep 0.4; done) | rtsort -h -t 3"
Sleep 0.5s
Enter
Sleep 4s
