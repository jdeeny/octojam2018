# Build dir
GEN:=build/

# Final Output
OUTPUT:=$(GEN)octojam2018.o8

SRC:=src/config.o8 src/math.o8 src/mem.o8 src/assert.o8 src/stack.o8 src/sprite_header.o8 src/screen_header.o8 src/trampoline.o8 src/main.o8 src/splash.o8 src/titlescreen.o8 src/audio.o8 src/input.o8 src/ecs.o8 src/text.o8 src/screen.o8 src/data.o8 src/sprite_data.o8 src/screen_data.o8 src/input_data.o8 src/font.o8 
#SRC:=src/config.o8 src/math.o8 src/mem.o8 src/assert.o8 src/stack.o8 src/trampoline.o8 src/main.o8 src/audio.o8 src/tetra.o8 build/tetra_header.o8 src/input.o8 src/ecs.o8 src/dialog.o8 src/data.o8 build/tetra_data.o8 src/input_data.o8 src/font.o8
#TETRA_SRC:=tetra/base.t4 tetra/test.t4

default: gen

.PHONY: $(OUTPUT)
gen: $(OUTPUT)

#.PHONY: $(TESTOUTPUT)
#test: $(TESTOUTPUT)

$(OUTPUT): $(SRC) Makefile #$(TETRA_SRC)
	mkdir -p $(GEN)
#	cat $(TETRA_SRC) > build/tetra_input.t4
#	tools/tetrac/target/debug/tetrac
	cat $(SRC) > $(OUTPUT)
	xclip -selection clipboard -i $(OUTPUT)
	echo .

clean:
	rm -rf $(GEN)
