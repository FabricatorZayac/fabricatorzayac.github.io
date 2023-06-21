.PHONY: all open clean

all: index.html

index.html: index.md
	pandoc index.md | cat header.html - > index.html

open: all
	surf index.html

clean:
	rm index.html
