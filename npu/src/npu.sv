module npu #(
    parameter int N
) (
    input logic clk,
    input logic rst,
    input logic start,
    output logic done,
    output logic signed [31:0] result

);



  logic signed [31:0] bias[1];
  logic signed [31:0] weights[N];
  logic signed [31:0] act[N];
  localparam int IDXW = N == 1 ? 1 : $clog2(N);
  logic [IDXW-1:0] i;

  logic signed [63:0] raw_mult;
  logic signed [31:0] mult;
  logic signed [63:0] acc;

  assign raw_mult = 64'(act[i]) * 64'(weights[i]);
  assign mult = raw_mult[55:24];

  always_ff @(posedge clk or posedge rst) begin
    if (rst) begin
      i <= 0;
      acc <= 0;
      result <= 0;
      done <= 0;
    end else if (start) begin
      done <= 0;
      if (32'(i) == N - 1) begin
        assign signed [63:0] total;

        total <= acc + 64'(mult) + 64'(bias[0]);
        acc <= 0;
        done <= 1;
      end else begin
        acc <= acc + 64'(mult);
        i   <= i + 1;
      end
    end

  end
endmodule
