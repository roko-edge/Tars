module npu #(
    parameter int N
) (
    input logic clk,
    input logic rst,
    input logic start,
    input logic signed [31:0] bias,
    output logic done,
    output logic signed [31:0] result

);



  logic signed [31:0] weights[N];
  logic signed [31:0] act[N];
  localparam int IDXW = N == 1 ? 1 : $clog2(N);
  logic [IDXW-1:0] i;

  logic signed [31:0] mult;
  logic signed [31:0] acc;

  assign mult = act[i] * weights[i];

  always_ff @(posedge clk or posedge rst) begin
    if (rst) begin
      i <= 0;
      acc <= 0;
      result <= 0;
      done <= 0;
    end else if (start) begin
      done <= 0;
      if (i == N - 1) begin
        result <= acc + mult + bias;
        acc <= 0;
        done <= 1;
      end else begin
        acc <= acc + mult;
      end
      i <= i + 1;
    end


  end
endmodule
