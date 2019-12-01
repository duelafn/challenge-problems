#!/usr/bin/perl
## CHEAT!: https://arstechnica.com/gaming/2018/12/for-the-first-time-ever-disney-posts-a-pixar-short-on-youtube-for-free/
use warnings;
use strict;
use autodie;
use 5.020;

my $ip_reg;
my @program;

my %ops =
  (
   addr => sub { "registers[$_[2]] = registers[$_[0]] + registers[$_[1]];" },
   addi => sub { "registers[$_[2]] = registers[$_[0]] + $_[1];" },
   mulr => sub { "registers[$_[2]] = registers[$_[0]] * registers[$_[1]];" },
   muli => sub { "registers[$_[2]] = registers[$_[0]] * $_[1];" },
   banr => sub { "registers[$_[2]] = registers[$_[0]] & registers[$_[1]];" },
   bani => sub { "registers[$_[2]] = registers[$_[0]] & $_[1];" },
   borr => sub { "registers[$_[2]] = registers[$_[0]] | registers[$_[1]];" },
   bori => sub { "registers[$_[2]] = registers[$_[0]] | $_[1];" },
   setr => sub { "registers[$_[2]] = registers[$_[0]];" },
   seti => sub { "registers[$_[2]] = $_[0];" },
   gtir => sub { "registers[$_[2]] = $_[0] > registers[$_[1]];" },
   gtri => sub { "registers[$_[2]] = registers[$_[0]] > $_[1];" },
   gtrr => sub { "registers[$_[2]] = registers[$_[0]] > registers[$_[1]];" },
   eqir => sub { "registers[$_[2]] = $_[0] == registers[$_[1]];" },
   eqri => sub { "registers[$_[2]] = registers[$_[0]] == $_[1];" },
   eqrr => sub { "registers[$_[2]] = registers[$_[0]] == registers[$_[1]];" }
  );

while (<>) {
  if (/#ip (\d+)/) {
    $ip_reg = $1;
  } elsif (/(\w+) (\d+) (\d+) (\d+)/) {
    push @program, $ops{$1}->($2, $3, $4);
  }
}

say "Creating day19.c";
open my $src, ">", "day19.c";

print $src <<EOC;
#include <stdio.h>

int registers[6];
void run(void);

void zero_registers(void) {
  for (int n = 0; n < 6; n += 1) { registers[n] = 0; }
}

int main(void) {
  zero_registers();
  run();
  printf("Part 1: %d\\n", registers[0]);
  zero_registers();
  registers[0] = 1;
  run();
  printf("Part 2: %d\\n", registers[0]);
  return 0;
}

EOC

my $ninsns = @program;
  print $src <<EOC;
void run(void) {
  int ip = 0;
  void *labels[] = {
EOC
say $src "    &&instr_$_," for 0 .. $ninsns - 1;
say $src "  };";
for (0 .. $ninsns - 1) {
  say $src "  instr_$_:";
  my $uses_ip = $program[$_] =~ m/registers\[$ip_reg\]/;
  my $sets_ip = $program[$_] =~ m/registers\[$ip_reg\] =/;
  say $src "  registers[$ip_reg] = ip;" if $uses_ip;
  say $src "  ", $program[$_];
  if ($sets_ip) {
    say $src "  ip = registers[$ip_reg] + 1;";
  } else {
    say $src "  ip += 1;";
  }
  say $src "  if (ip >= $ninsns) { return; }";
  say $src "  goto *labels[ip];";
}
say $src "}";
close $src;

say "Compiling day19.c";
system "gcc -std=gnu11 -O3 -march=native -Wall -Wextra -o day19 day19.c";
say "Running day19 executable";
system "./day19";
