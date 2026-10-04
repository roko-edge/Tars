module pe (
    input logic clk,
    input logic rst,
    input logic init,
    input logic en,
    input logic signed [31:0] w,
    input logic signed [31:0] a,
    output logic signed [63:0] accumulator
);
  logic signed [63:0] raw_mult;
  assign raw_mult = 64'(a) * 64'(w);
  always_ff @(posedge clk or posedge rst) begin
    if (rst) begin
      accumulator <= 0;
    end else if (en) begin
      if (init) begin
        accumulator <= (raw_mult >>> 24);
      end else begin
        //q8.24*q8.24 == q16.48
        //>>> is the arithmetic right shift,preserve the sign
        //The pattern is 24 bit after the dot,the shift preserve this
        accumulator <= accumulator + (raw_mult >>> 24);
      end
    end
  end
endmodule
