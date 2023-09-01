SRCS     := $(shell find . -name "*.md" ! -name "README.md")
TARGETS  := $(SRCS:%.md=%.html)

.PHONY: all clean serve

all: $(TARGETS)

%.html: %.md style.css skeleton.html
	pandoc --template=skeleton.html $< -o $@

serve: all
	./serve

clean:
	rm -f $(TARGETS)
