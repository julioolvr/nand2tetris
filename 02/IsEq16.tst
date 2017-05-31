// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/01/Mux8Way16.tst

load IsEq16.hdl,
output-file IsEq16.out,
compare-to IsEq16.cmp,
output-list a%B1.16.1 b%B1.16.1 out%B3.1.3;

set a 0,
set b 0,
eval,
output;

set a %B0000000000001000;
set b %B0000000000001000;
eval,
output;

set a %B0001000000000000;
set b %B0001000000000000;
eval,
output;

set a %B1111111111111111;
set b %B1111111111111111;
eval,
output;

set a %B0000000000000000;
set b %B0000000000001000;
eval,
output;

set a %B0000000000000000;
set b %B0000100000000000;
eval,
output;

set a %B1111111111111111;
set b %B0000000000000000;
eval,
output;