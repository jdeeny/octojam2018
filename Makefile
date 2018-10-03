# Build dir
GEN:=build/

# Final Output
OUTPUT:=$(GEN)octojam2018.o8

SRC:=mem.o8 trampoline.o8 main.o8 input.o8 ecs.o8 audio.o8 fardata.o8

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
