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
    for (int j = 0; j < OUT; ++j) begin
      for (int i = 0; i < IN; i += 2) begin
        rst   = 1;
        start = 0;
        #10;
        rst = 0;
        dut.a[0] = data[i];
        dut.a[1] = data[i+1];
        start = 1;
        wait (done);
        $display("entries: %0d %0d and the result is %0d\n", data[i], data[i+1], result[j]);
      end
    end
    $finish;
  end
endmodule
