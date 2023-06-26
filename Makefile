SRCS     := $(shell find . -name "*.md" ! -name "README.md")
TARGETS  := $(SRCS:%.md=%.html)

.PHONY: all open clean

all: $(TARGETS)

%.html: %.md
	pandoc $< | cat header.html - > $@

open: all
	surf index.html

clean:
	rm $(TARGETS)
