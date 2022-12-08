
HS  := $(wildcard *.hs)
EXE := $(HS:%.hs=%)

all: ${EXE}
	@rm -f *.hi *.o

clean:
	rm -f *.hi *.o
	rm -f [0-9][0-9]

%: %.rs
	rustc -O $^
	@strip $@

%: %.hs
	ghc $^
	@strip $@
