# Build dir
GEN:=build/

# Final Output
OUTPUT:=$(GEN)octojam2018.o8

STATES:=src/states/charedit.o8 src/states/endscreen.o8 src/states/gameplay.o8 src/states/splash.o8 src/states/titlescreen.o8

SRC:=src/config.o8 src/math.o8 src/mem.o8 src/assert.o8 src/stack.o8 src/sprite_header.o8 src/screen_header.o8 src/ai.o8 src/prefab_header.o8 src/rendererheader.o8  src/state_header.o8 src/trampoline.o8  src/main.o8 src/ecs.o8 src/renderer.o8 src/worldgen.o8 src/gamestate.o8 $(STATES) src/audio.o8 src/input.o8 src/text.o8 src/screen.o8 src/font.o8 src/data.o8 src/sprite_data.o8 src/screen_data.o8 src/input_data.o8 src/prefab_data.o8 src/rendererdata.o8 src/worldgen_data.o8

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
