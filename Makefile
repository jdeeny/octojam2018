# Build dir
GEN:=build/

# Final Output
OUTPUT:=$(GEN)octojam2018.o8

SRC:=src/config.o8 src/mem.o8 src/assert.o8 src/stack.o8 src/trampoline.o8 src/tetra.o8 src/main.o8 src/input.o8 src/ecs.o8 src/audio.o8 src/data.o8

default: gen

.PHONY: $(OUTPUT)
gen: $(OUTPUT)

#.PHONY: $(TESTOUTPUT)
#test: $(TESTOUTPUT)

$(OUTPUT): $(SRC) $(TETRAGENOUT) Makefile
	mkdir -p $(GEN)
	cat $(SRC) > $(OUTPUT)
	xclip -selection clipboard -i $(OUTPUT)
	echo .

clean:
	rm -rf $(GEN)
