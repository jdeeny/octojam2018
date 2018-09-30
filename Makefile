# Build dir
GEN:=build/

# Final Output
OUTPUT:=$(GEN)octojam2017.o8
BINOUTPUT:=$(OUTPUT).bin

TESTOUTPUT:=$(GEN)octojam2017-test.o8
TESTBINOUTPUT:=$(TESTOUTPUT).bin
TETRASRC:=tetra/base.t4
TETRAGEN:=$(GEN)tetra.t4
TETRACODE:=$(GEN)tetra-code.o8
TETRAFAR:=$(GEN)tetra-data.o8
TETRAGENOUT:=$(TETRAFAR) $(TETRACODE)

MATHDIR:=math/
MATHSRC:=$(MATHDIR)math.o8 $(MATHDIR)mul.o8 $(MATHDIR)div.o8

TEXTSRC:=text/glyphs.txt text/strings.txt
TEXTGEN:=$(GEN)text.out
TEXTCODE:=$(GEN)text-code.o8
TEXTFAR:=$(GEN)text-data.o8
TEXTGENOUT:=$(TEXTCODE) $(TEXTFAR)

GRAPHICS:=graphics/header.o8 graphics/tile.o8 graphics/tilebuffer.o8 graphics/display.o8 graphics/buffer.o8 graphics/displayobject.o8 graphics/bufferaligner.o8 graphics/compositor.o8

# Sourcecode, divided into sections
# Main, not used for tests
MAIN:=main.o8
# Headers
HEADERS:=bookends/header.o8 registers.o8 config.o8
# All the code, in order
CODE:=bookends/code-header.o8 assert.o8 mem.o8 stack.o8 $(MATHSRC) memcopy.o8 lists.o8 heap.o8 trampoline.o8 tetra/tetra-base.o8 $(TETRACODE) tetra/tetra.o8 $(TEXTCODE) $(GRAPHICS) text/text-render.o8 text/numbers.o8
# Everything that is > 4k
FAR:=bookends/far-header.o8 $(TETRAFAR) $(TEXTFAR) far.o8 bookends/far-footer.o8

# Combine all octo files in test dir
TESTDIR:=tests/
TESTMAIN:=tests.o8
TESTS := $(TESTDIR)$(TESTMAIN) $(shell find $(TESTDIR) -name '*.o8' -and -not -name '$(TESTMAIN)' )

# Set of all Octo source files
SRC=$(HEADERS) $(FAR) $(CODE) $(MAIN) bookends/code-footer.o8 bookends/footer.o8
TESTSRC=$(HEADERS) $(FAR) $(CODE) $(TESTS) bookends/code-footer.o8 bookends/footer.o8

default: gen

.PHONY: $(OUTPUT)
gen: $(OUTPUT)

.PHONY: $(TESTOUTPUT)
test: $(TESTOUTPUT)

$(TETRAGEN): $(TETRASRC)
	mkdir -p $(GEN)
	cat $(TETRASRC) > $(TETRAGEN)

$(TEXTGEN): $(TEXTSRC)
	mkdir -p $(GEN)
	cat $(TEXTSRC) > $(TEXTGEN)


$(TETRACODE): $(TETRAGEN)
$(TETRAFAR): $(TETRAGEN)
$(TEXTCODE): $(TEXTGEN)
$(TEXTFAR): $(TEXTGEN)

$(TEXTGENOUT): $(TEXTSRC) text/text.py Makefile
	mkdir -p $(GEN)
	python3 text/text.py $(TEXTGEN) $(TEXTCODE) $(TEXTFAR)

$(TETRAGENOUT): $(TETRASRC) tetra/tetra.py tetra/tetra-base.o8 Makefile
	mkdir -p $(GEN)
	python3 tetra/tetra.py $(TETRASRC) $(TETRACODE) $(TETRAFAR)

$(OUTPUT): $(SRC) $(TETRAGENOUT) Makefile
	mkdir -p $(GEN)
	cat $(SRC) > $(OUTPUT)
	xclip -selection clipboard -i $(OUTPUT)
	echo .
	ci/Octo/octo $(OUTPUT) > $(BINOUTPUT)

$(TESTOUTPUT): $(TESTSRC) Makefile
	mkdir -p $(GEN)
	cat $(TESTSRC) > $(TESTOUTPUT)
	xclip -selection clipboard -i $(TESTOUTPUT)
	echo .
	ci/Octo/octo $(TESTOUTPUT) > $(BINOUTPUT)

clean:
	rm -rf $(GEN)
