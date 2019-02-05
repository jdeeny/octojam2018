# Build dir
GEN:=build/

# Final Output
OUTPUT:=$(GEN)octojam2018.o8

AUDIO:= src/audio/audio.o8
STATES:=src/states/charedit.o8 src/states/endscreen.o8 src/states/gameplay.o8 src/states/splash.o8 src/states/titlescreen.o8 src/states/credits.o8 src/states/dialog.o8 src/states/newlevel.o8 src/states/enemyinfo.o8 src/states/beastiary.o8 src/states/iteminfo.o8 src/states/itemiary.o8 src/states/soundtest.o8
GAMEPLAY:=src/gameplay/gamestate.o8 src/gameplay/ecs.o8 src/gameplay/ai.o8 src/gameplay/combat.o8
UI:=src/ui/text.o8 src/ui/screen.o8
UTIL:= src/util/assert.o8 src/util/math.o8 src/util/mem.o8 src/util/stack.o8 src/util/trampoline.o8 src/util/random.o8
INPUT:=src/input/input.o8
RENDERER:=src/renderer/renderer.o8
PREFABS:=src/prefab_data.o8

HEADERS:=src/config.o8 $(UTIL) src/ui/screen_header.o8 src/audio/sample_header.o8 src/renderer/rendererheader.o8 src/prefab_header.o8 src/states/state_header.o8
CODE:=src/main.o8 $(AUDIO) $(GAMEPLAY) $(UI) $(INPUT) $(STATES) $(RENDERER) src/worldgen.o8
FARDATA:= src/sprite_data.o8 src/data.o8 $(PREFABS) src/audio/sample_data.o8 src/input/input_data.o8 src/states/state_data.o8 src/initializers.o8 src/ui/screen_data.o8 src/renderer/rendererdata.o8 src/worldgen_data.o8

SRC:=$(HEADERS) $(CODE) $(FARDATA)

default: gen

.PHONY: $(OUTPUT)
gen: $(OUTPUT)

$(OUTPUT): $(SRC) Makefile
	mkdir -p $(GEN)
	cat $(SRC) > $(OUTPUT)
	xclip -selection clipboard -i $(OUTPUT)
	echo .

clean:
	rm -rf $(GEN)
