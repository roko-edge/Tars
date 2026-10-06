module npu #(
    parameter int IN  = 2,
    parameter int OUT = 2

) (
    input logic clk,
    input logic rst,
    input logic start,
    output logic done,
    output logic signed [31:0] result[OUT]

);
  typedef enum logic {
    IDLE = 0,
    BUSY = 1
  } state_t;
  state_t state;

  logic signed [31:0] b[OUT];
  logic signed [31:0] w[IN*OUT];
  logic signed [31:0] a[IN];

  logic unsigned [$clog2(OUT):0] j;
  logic unsigned [$clog2(IN):0] i;

  logic signed [63:0] pe_acc;

  pe u_pe (
      .clk        (clk),
      .rst        (rst),
      .init       (i == 0),
      .en         (state == BUSY),
      .a          (a[i]),
      .w          (w[j*IN+i]),
      .accumulator(pe_acc)
  );

  logic signed [63:0] sum64;
  logic signed [31:0] sum32;

  assign sum64 = pe_acc + 64'(b[j]);

  always_comb begin
    if (sum64 == 64'(signed'(sum64[31:0]))) begin
      sum32 = sum64[31:0];
    end else begin
      sum32 = sum64[63] ? 32'sh8000_0000 : 32'sh7FFF_FFFF;
    end
  end

  always_ff @(posedge clk or posedge rst) begin
    if (rst) begin
      j <= 0;
      i <= 0;
      for (int k = 0; k < OUT; ++k) begin
        result[k] <= 0;
      end
      done  <= 0;
      state <= IDLE;
    end else begin
      unique case (state)
        IDLE: begin
          if (start) begin
            done <= 0;
            state <= BUSY;
            i <= 0;
            j <= 0;
          end
        end
        BUSY: begin
          if (32'(i) == IN) begin

            result[j] <= sum32;
            i <= 0;
            if (32'(j) == OUT - 1) begin
              state <= IDLE;
              done <= 1;
              j <= 0;
            end else begin
              j <= j + 1;

            end
          end else begin
            i <= i + 1;
          end

        end
      endcase

    end
  end
endmodule
