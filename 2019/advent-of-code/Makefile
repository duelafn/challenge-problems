
DD  ?= $(shell date +%d)

TIMEFMT = %a, %d %b %Y %T %z

define SET_START
  use Time::Piece;
  BEGIN { $$t1 = localtime; }
  s/\bDD\b/${DD}/g;
  s/\W+Time Start:\K\s*$$/sprintf(" %s\n", $$t1->strftime("${TIMEFMT}"))/e;
endef
export SET_START

define SET_STEP1
  use Time::Piece;
  BEGIN { $$t1 = localtime; }
  $$t0 = Time::Piece->strptime($$1, "${TIMEFMT}") if /\W+Time Start:\s*(.*)/;
  s/\W+Time Finish 1:\K.*$$/sprintf(" %s (%s)", $$t1->strftime("${TIMEFMT}"), ($$t1-$$t0)->pretty)/e;
endef
export SET_STEP1

define SET_STEP2
  use Time::Piece;
  BEGIN { $$t2 = localtime; }
  $$t0 = Time::Piece->strptime($$1, "${TIMEFMT}") if /\W+Time Start:\s*(.*)/;
  $$t1 = Time::Piece->strptime($$1, "${TIMEFMT}") if /\W+Time Finish 1:\s*(.*?)\s*\(/;
  s/\W+Time Finish 2:\K.*$$/sprintf(" %s (%s)", $$t2->strftime("${TIMEFMT}"), ($$t2-$$t1)->pretty)/e;
  s/\W+Time Total:\K.*$$/" " . ($$t2-$$t0)->pretty/e;
endef
export SET_STEP2


start:
	mkdir -p ${DD}/src
	cp -i _tmpl.rs  ${DD}/src/main.rs
	cp -i _Cargo.toml   ${DD}/Cargo.toml
	perl -pi -E "$$SET_START" ${DD}/src/main.rs ${DD}/Cargo.toml
	touch ${DD}/${DD}.in
	emacsclient --no-wait ${DD}/src/main.rs
	FvwmCommand 'GotoPage 1 0' 'WindowId root 1 WarpToWindow 50 50'

step1:
	perl -pi -E "$$SET_STEP1" ${DD}/src/main.rs
	git add ${DD}

step2:
	perl -pi -E "$$SET_STEP2" ${DD}/src/main.rs


clean:
	rm -rf */target
