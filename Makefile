SRCS     := $(shell find . -name "*.md" ! -name "README.md")
TARGETS  := $(SRCS:%.md=%.html)

.PHONY: all clean local

all: $(TARGETS)

%.html: %.md
	pandoc $< | cat header.html - > $@

local: all
	./localrun

clean:
	rm -f $(TARGETS)
