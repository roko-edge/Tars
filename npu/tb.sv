`timescale 1ns / 1ps
module tb;
  logic clk;
  logic rst;
  logic start;

  logic signed [31:0] bias;
  logic signed [31:0] result;
  logic done;
  npu #(
      .N(4)
  ) dut (
      .clk(clk),
      .rst(rst),
      .start(start),
      .bias(bias),
      .done(done),
      .result(result)
  );

  always #5 clk = ~clk;

  initial begin
    clk   = 0;
    rst   = 1;
    start = 0;
    bias  = 0;
    #10;
    rst = 0;

    $readmemh("activations.mem", dut.act);
    $readmemh("weights.mem", dut.weights);

    bias  = 7;
    start = 1;
    wait (done);
    $display("the result is %0d\n", result);
    $finish;
  end
endmodule
