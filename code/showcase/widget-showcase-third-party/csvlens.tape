# A VHS tape. See https://github.com/charmbracelet/vhs

# Assumes you have installed csvlens https://github.com/YS-L/csvlens
# and grabbed a file named pokemon.csv (e.g. https://gist.github.com/armgilles/194bcff35001e7eb53a2a8b441e8b2c6)
# (should be in ~/tmp/pokemon.csv)
Output "csvlens.gif"
Set Theme "Aardvark Blue"
Set Width 1200
Set Height 800
Hide
Type "csvlens ~/tmp/pokemon.csv" Enter
Sleep 2s
Show
Sleep 2s
Down @0.2s 23
Sleep 2s
Type @0.3s "/Flying"
Sleep 2s
Enter @2s
Type @0.5s "nnnnnnn"
Sleep 2s
Type @0.3s "&Bug" 
Sleep 2s
Enter @2s
Down @0.5s 7
Sleep 2s
Type @0.3s "*Name|Leg"
Sleep 2s
Enter @2s
Sleep 2s
Hide
Type "q"