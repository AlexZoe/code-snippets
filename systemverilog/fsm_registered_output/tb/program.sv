program test(
  input bit clk,
  input logic reset,
  input logic red,
  input logic yellow,
  input logic green
);

  initial begin
    wait (red == 1);
    assert(!yellow);
    assert(!green);
    wait (yellow == 1);
    wait (green == 1);
  end

endprogram
