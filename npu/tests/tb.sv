`timescale 1ns / 1ps
module tb;
  logic clk;
  logic rst;
  logic start;
  localparam int IN  = 2;
  localparam int OUT = 1;
  logic signed [31:0] result[OUT];
  logic done;
  npu #(
      .IN (IN),
      .OUT(OUT)
  ) dut (
      .clk(clk),
      .rst(rst),
      .start(start),
      .done(done),
      .result(result)
  );

  always #5 clk = ~clk;
  logic signed [31:0] data[8];

  initial begin
    $dumpfile("build/dump.vcd");
    $dumpvars(0, tb);

    clk = 0;

    $readmemh("data/bias.mem", dut.b);
    $readmemh("data/weights.mem", dut.w);
    $readmemh("data/activations.mem", data);



    for (int s = 0; s < 4; ++s) begin
      rst   = 1;
      start = 0;
      #10;
      rst = 0;
      dut.a[0] = data[s*2];
      dut.a[1] = data[s*2+1];

      #10;
      start = 1;
      #10;
      start = 0;
      wait (done);
      $display("Entries:%d %d the result is %d\n", data[s*2], data[s*2+1], result[0]);
    end

    $finish;
  end
endmodule
