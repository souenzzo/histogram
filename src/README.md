# histogram
A small CLI utility to make histograms

To compile
----------
Just make it
```bash
	$ make
```

To 'Install'
------------
Copy for some folder in your PATH and make it executable
```bash
	$ echo $PATH
	# cp histogram /usr/bin/
	# chmod +x /usr/bin/histogram
```

To use
------
Pipe numeric data (integer only for now) to stdin and it will make a histogram with it
Arguments
```
	-b --buffer=[integer] | Set number of max input lines
	-d --delta=[integer]  | Set row's delta
	-f --fancy            | Plot awesome graphs!
	-h --help             | Print help text and exit.
	-H --human            | Print same usefull extra information
	-i --input=[file]     | Set a file to input instead stdin
	-o --output=[file]    | Set a file to output instead stdout
	-v --version          | Print version and exit
```
