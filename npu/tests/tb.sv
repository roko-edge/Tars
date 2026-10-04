`timescale 1ns / 1ps
module tb;
  logic clk;
  logic rst;
  logic start;

  logic signed [31:0] result;
  logic done;
  npu #(
      .N(2)
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
    for (int i = 0; i < 8; i += 2) begin
      rst   = 1;
      start = 0;
      #10;
      rst = 0;
      dut.a[0] = data[i];
      dut.a[1] = data[i+1];
      start = 1;
      wait (done);
      $display("entries: %0d %0d and the result is %0d\n", data[i], data[i+1], result);
    end
    $finish;
  end
endmodule
