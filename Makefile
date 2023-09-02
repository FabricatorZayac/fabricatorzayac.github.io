SRCS     := $(shell find . -name "*.md" ! -name "README.md")
TARGETS  := $(SRCS:%.md=%.html)
TEMPLATE := skeleton.html

.PHONY: all clean serve

all: $(TARGETS)

%.html: %.md $(TEMPLATE)
	pandoc --template=$(TEMPLATE) $< -o $@

serve: all
	./serve

clean:
	rm -f $(TARGETS)

# DON'T USE OUTSIDE OF PAGES DEPLOYMENT
purge-deploy: all
	rm -f $(SRCS)
