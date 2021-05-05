module traffic_light_tb;

  bit clk;
  bit reset;
  bit green, red, yellow;
  localparam PERIOD = 10;

  always #(PERIOD/2) clk = ~clk;

  initial begin
    reset = 1;
    #(PERIOD * 5) reset = 0;
  end

  initial begin
    $dumpfile("wave.vcd");
    $dumpvars;
  end

  test t1(.*);

  initial begin
    #(PERIOD * 15) $finish;
  end

  traffic_light DUT (.*);

endmodule
